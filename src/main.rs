use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use clap::{ArgGroup, Parser, Subcommand};
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
    #[command(group(
        ArgGroup::new("code_set_rules")
        .required(false)
        .args(["code_128_compact", "force_code_set"]),
    ))]
    #[command(group(
        ArgGroup::new("data_source")
        .required(true)
        .args(["data", "data_file"]),
    ))]
    #[command(group(
        ArgGroup::new("data_matrix_encoding")
        .required(false)
        .args(["data_matrix_compact","force_c40"]),
    ))]
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

        /// Specifies what degree of error correction to use, for example in QR Codes.
        /// Type depends on the encoder. For example for QR codes it's (L,M,Q,H).
        /// For Aztec it is of type u32, representing the minimal percentage of error correction words.
        /// For PDF417 it is of type u8, valid values being 0 to 8.
        /// Note: an Aztec symbol should have a minimum of 25% EC words.
        #[arg(long, verbatim_doc_comment)]
        error_correction: Option<String>,

        /// Specifies what character encoding to use where applicable.
        #[arg(long)]
        character_set: Option<String>,

        /// Specifies whether to use compact mode for Data Matrix.
        /// The compact encoding mode also supports the encoding of characters that are not in the ISO-8859-1
        /// character set via ECIs.
        /// Please note that in that case, the most compact character encoding is chosen for characters in
        /// the input that are not in the ISO-8859-1 character set. Based on experience, some scanners do not
        /// support encodings like cp-1256 (Arabic). In such cases the encoding can be forced to UTF-8 by
        /// means of the #CHARACTER_SET encoding hint.
        /// Compact encoding also provides GS1-FNC1 support when #GS1_FORMAT is selected. In this case
        /// group-separator character (ASCII 29 decimal) can be used to encode the positions of FNC1 codewords
        /// for the purpose of delimiting AIs.
        #[arg(long, verbatim_doc_comment)]
        data_matrix_compact: Option<bool>,

        /// Specifies margin, in pixels, to use when generating the barcode.
        /// The meaning can vary
        /// by format; for example it controls margin before and after the barcode horizontally for
        /// most 1D formats.
        #[arg(long, verbatim_doc_comment)]
        margin: Option<String>,

        /**
         Specifies whether to use compact mode for PDF417.
        */
        #[arg(long)]
        pdf_417_compact: Option<bool>,

        /**
         Specifies what compaction mode to use for PDF417
         AUTO = 0,
         TEXT = 1,
         BYTE = 2,
         NUMERIC = 3
        */
        #[arg(long)]
        pdf_417_compaction: Option<String>,

        /// Specifies whether to automatically insert ECIs when encoding PDF417.
        /// Please note that in that case, the most compact character encoding is chosen for characters in
        /// the input that are not in the ISO-8859-1 character set. Based on experience, some scanners do not
        /// support encodings like cp-1256 (Arabic). In such cases the encoding can be forced to UTF-8 by
        /// means of the #CHARACTER_SET encoding hint.
        #[arg(long, verbatim_doc_comment)]
        pdf_417_auto_eci: Option<bool>,

        /// Specifies the required number of layers for an Aztec code.
        /// A negative number (-1, -2, -3, -4) specifies a compact Aztec code.
        /// 0 indicates to use the minimum number of layers (the default).
        /// A positive number (1, 2, .. 32) specifies a normal (non-compact) Aztec code.
        #[arg(long, verbatim_doc_comment)]
        aztec_layers: Option<i32>,

        /**
         Specifies the exact version of QR code to be encoded.
        */
        #[arg(long)]
        qr_version: Option<String>,

        /// Specifies the QR code mask pattern to be used. Allowed values are
        /// 0..8. By default the code will automatically select
        /// the optimal mask pattern.
        #[arg(long, verbatim_doc_comment)]
        qr_mask_pattern: Option<String>,

        /// Specifies whether to use compact mode for QR code.
        /// Please note that when compaction is performed, the most compact character encoding is chosen
        /// for characters in the input that are not in the ISO-8859-1 character set. Based on experience,
        /// some scanners do not support encodings like cp-1256 (Arabic). In such cases the encoding can
        /// be forced to UTF-8 by means of the #CHARACTER_SET encoding hint.
        #[arg(long, verbatim_doc_comment)]
        qr_compact: Option<bool>,

        /**
         Specifies whether the data should be encoded to the GS1 standard/
        */
        #[arg(long)]
        gs1_format: Option<bool>,

        /// Forces which encoding will be used. Currently only used for Code-128 code sets.
        /// Valid values are "A", "B", "C".
        #[arg(long, verbatim_doc_comment)]
        force_code_set: Option<String>,

        /**
         Forces C40 encoding for data-matrix. This
        */
        #[arg(long)]
        force_c40: Option<bool>,

        /**
         Specifies whether to use compact mode for Code-128 code.
         This can yield slightly smaller bar codes.
        */
        #[arg(long)]
        code_128_compact: Option<bool>,
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
            error_correction,
            character_set,
            data_matrix_compact,
            margin,
            pdf_417_compact,
            pdf_417_compaction,
            pdf_417_auto_eci,
            aztec_layers,
            qr_version,
            qr_mask_pattern,
            qr_compact,
            gs1_format,
            force_code_set,
            force_c40,
            code_128_compact,
        } => encode_command(
            &cli.file_name,
            barcode_type,
            width,
            height,
            data,
            data_file,
            error_correction,
            character_set,
            data_matrix_compact,
            margin,
            pdf_417_compact,
            pdf_417_compaction,
            pdf_417_auto_eci,
            aztec_layers,
            qr_version,
            qr_mask_pattern,
            qr_compact,
            gs1_format,
            force_code_set,
            force_c40,
            code_128_compact,
        ),
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

    error_correction: &Option<String>,

    character_set: &Option<String>,

    data_matrix_compact: &Option<bool>,

    margin: &Option<String>,

    pdf_417_compact: &Option<bool>,

    pdf_417_compaction: &Option<String>,

    pdf_417_auto_eci: &Option<bool>,

    aztec_layers: &Option<i32>,

    qr_version: &Option<String>,

    qr_mask_pattern: &Option<String>,

    qr_compact: &Option<bool>,

    gs1_format: &Option<bool>,

    force_code_set: &Option<String>,

    force_c40: &Option<bool>,

    code_128_compact: &Option<bool>,
) {
    // if data.is_none() && data_file.is_none() {
    //     println!("must provide either data string or data file");
    //     return;
    // }
    // if data.is_some() && data_file.is_some() {
    //     println!("provide only data string or data file");
    //     return;
    // }

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

    let mut hints: rxing::EncodingHintDictionary = HashMap::new();

    if let Some(ec) = error_correction {
        hints.insert(
            rxing::EncodeHintType::ERROR_CORRECTION,
            rxing::EncodeHintValue::ErrorCorrection(ec.to_owned()),
        );
    }

    if let Some(character_set) = character_set {
        hints.insert(
            rxing::EncodeHintType::CHARACTER_SET,
            rxing::EncodeHintValue::CharacterSet(character_set.to_owned()),
        );
    }

    if let Some(data_matrix_compact) = data_matrix_compact {
        hints.insert(
            rxing::EncodeHintType::DATA_MATRIX_COMPACT,
            rxing::EncodeHintValue::DataMatrixCompact(*data_matrix_compact),
        );
    }

    if let Some(margin) = margin {
        hints.insert(
            rxing::EncodeHintType::MARGIN,
            rxing::EncodeHintValue::Margin(margin.to_owned()),
        );
    }

    if let Some(pdf_417_compact) = pdf_417_compact {
        hints.insert(
            rxing::EncodeHintType::PDF417_COMPACT,
            rxing::EncodeHintValue::Pdf417Compact(pdf_417_compact.to_string()),
        );
    }

    if let Some(pdf_417_compaction) = pdf_417_compaction {
        hints.insert(
            rxing::EncodeHintType::PDF417_COMPACTION,
            rxing::EncodeHintValue::Pdf417Compaction(pdf_417_compaction.to_owned()),
        );
    }

    if let Some(pdf_417_auto_eci) = pdf_417_auto_eci {
        hints.insert(
            rxing::EncodeHintType::PDF417_AUTO_ECI,
            rxing::EncodeHintValue::Pdf417AutoEci(pdf_417_auto_eci.to_string()),
        );
    }

    if let Some(aztec_layers) = aztec_layers {
        hints.insert(
            rxing::EncodeHintType::AZTEC_LAYERS,
            rxing::EncodeHintValue::AztecLayers(*aztec_layers),
        );
    }

    if let Some(qr_version) = qr_version {
        hints.insert(
            rxing::EncodeHintType::QR_VERSION,
            rxing::EncodeHintValue::QrVersion(qr_version.to_owned()),
        );
    }

    if let Some(qr_mask_pattern) = qr_mask_pattern {
        hints.insert(
            rxing::EncodeHintType::QR_MASK_PATTERN,
            rxing::EncodeHintValue::QrMaskPattern(qr_mask_pattern.to_owned()),
        );
    }

    if let Some(qr_compact) = qr_compact {
        hints.insert(
            rxing::EncodeHintType::QR_COMPACT,
            rxing::EncodeHintValue::QrCompact(qr_compact.to_string()),
        );
    }

    if let Some(gs1_format) = gs1_format {
        hints.insert(
            rxing::EncodeHintType::GS1_FORMAT,
            rxing::EncodeHintValue::Gs1Format(*gs1_format),
        );
    }

    if let Some(force_code_set) = force_code_set {
        hints.insert(
            rxing::EncodeHintType::FORCE_CODE_SET,
            rxing::EncodeHintValue::ForceCodeSet(force_code_set.to_owned()),
        );
    }

    if let Some(force_c40) = force_c40 {
        hints.insert(
            rxing::EncodeHintType::FORCE_C40,
            rxing::EncodeHintValue::ForceC40(*force_c40),
        );
    }

    if let Some(code_128_compact) = code_128_compact {
        hints.insert(
            rxing::EncodeHintType::CODE128_COMPACT,
            rxing::EncodeHintValue::Code128Compact(*code_128_compact),
        );
    }

    println!("Encode: file_name: {}, barcode_type: {}, width: {:?}, height: {:?}, data: '{:?}', data_file: {:?}", file_name, barcode_type, width, height, data, data_file);
    let writer = MultiFormatWriter::default();
    match writer.encode_with_hints(
        &input_data,
        barcode_type,
        *width as i32,
        *height as i32,
        &hints,
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
