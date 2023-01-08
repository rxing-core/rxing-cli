use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use clap::{Parser, Subcommand};
use rxing::{BarcodeFormat, MultiFormatWriter, Writer};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    file_name: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Decode {
        #[arg(short, long)]
        try_harder: bool,
        #[arg(short, long)]
        decode_multi: bool,
        #[arg(short, long, value_enum)]
        barcode_types: Option<Vec<BarcodeFormat>>,
    },
    Encode {
        barcode_type: BarcodeFormat,
        #[arg(long)]
        width: u32,
        #[arg(long)]
        height: u32,
        #[arg(short, long)]
        data: Option<String>,
        #[arg(long)]
        data_file: Option<String>,
    },
}

fn main() {
    println!("rxing-cli");
    let cli = Args::parse();
    match &cli.command {
        Commands::Decode {
            try_harder,
            decode_multi,
            barcode_types,
        } => decode_command(&cli.file_name, try_harder, decode_multi, barcode_types),
        Commands::Encode {
            barcode_type,
            width,
            height,
            data,
            data_file,
        } => encode_command(&cli.file_name, barcode_type, width, height, data, data_file),
    }
}

fn decode_command(
    file_name: &str,
    try_harder: &bool,
    decode_multi: &bool,
    barcode_types: &Option<Vec<BarcodeFormat>>,
) {
    println!(
        "Decode '{}' with: try_harder: {}, decode_multi: {}, barcode_types: {:?}",
        file_name, try_harder, decode_multi, barcode_types
    );
    let mut hints: rxing::DecodingHintDictionary = HashMap::new();
    if !try_harder {
        hints.insert(
            rxing::DecodeHintType::TRY_HARDER,
            rxing::DecodeHintValue::TryHarder(false),
        );
    }
    if let Some(barcode_type) = barcode_types {
        hints.insert(
            rxing::DecodeHintType::POSSIBLE_FORMATS,
            rxing::DecodeHintValue::PossibleFormats(HashSet::from_iter(
                barcode_type.iter().copied(),
            )),
        );
    }

    if *decode_multi {
        let results = rxing::helpers::detect_multiple_in_file_with_hints(file_name, &mut hints);
        match results {
            Ok(result_array) => {
                println!("Found {} results", result_array.len());
                for (i, result) in result_array.into_iter().enumerate() {
                    println!("Result {}: ({}) {}", i, result.getBarcodeFormat(), result);
                }
            }
            Err(search_err) => {
                println!(
                    "Error while attempting to locate multiple barcodes in '{}': {}",
                    file_name, search_err
                );
            }
        }
    } else {
        let result = rxing::helpers::detect_in_file_with_hints(file_name, None, &mut hints);
        match result {
            Ok(result) => {
                println!(
                    "Detection result: \n({}) {}",
                    result.getBarcodeFormat(),
                    result
                );
            }
            Err(search_err) => {
                println!(
                    "Error while attempting to locate barcode in '{}': {}",
                    file_name, search_err
                );
            }
        }
    }
}

fn encode_command(
    file_name: &str,
    barcode_type: &BarcodeFormat,
    width: &u32,
    height: &u32,
    data: &Option<String>,
    data_file: &Option<String>,
) {
    if data.is_none() && data_file.is_none() {
        println!("must provide either data string or data file");
        return;
    }
    if data.is_some() && data_file.is_some() {
        println!("provide only data string or data file");
        return;
    }

    let input_data = if let Some(df) = data_file {
        let path_from = PathBuf::from(df);
        if path_from.exists() {
            let Ok(fl) = std::fs::File::open(path_from) else {
                println!("file cannot be opened");
                return;
            };
            std::io::read_to_string(fl).expect("file should read")
        } else {
            println!("{} does not exist", df);
            return;
        }
    } else if let Some(ds) = data {
        ds.to_owned()
    } else {
        println!("Unknown error getting data");
        return;
    };

    println!("Encode: file_name: {}, barcode_type: {}, width: {:?}, height: {:?}, data: '{:?}', data_file: {:?}", file_name, barcode_type, width, height, data, data_file);
    let writer = MultiFormatWriter::default();
    match writer.encode_with_hints(
        &input_data,
        barcode_type,
        *width as i32,
        *height as i32,
        &HashMap::new(),
    ) {
        Ok(result) => {
            println!("Encode successful, saving...");
            match rxing::helpers::save_image(file_name, &result) {
                Ok(_) => println!("Saved to '{}'", file_name),
                Err(error) => println!("Could not save '{}': {}", file_name, error),
            }
        }
        Err(encode_error) => println!("Couldn't encode: {}", encode_error),
    }
}
