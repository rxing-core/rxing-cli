# Archive
This repository will be archived, the functionality is being moved into the [rxing repository workspace](https://github.com/rxing-core/rxing), the crate will be managed from within that repository, the procedure to use it will not chnage.

# rxing-cli
A command line interface for rxing supporting encoding and decoding of barcode data.

## Full documentation
`rxing-cli help`
`rxing-cli help encode`
`rxing-cli help decode`

## Instalation 
`cargo install rxing-cli`

## Example Encode
`rxing-cli test_image.jpg encode --width 500 --height 500 --data "Sample Data and TEST Data" qrcode`

## Example Decode
`rxing-cli test_image.jpg decode`

## Example Multi Barcode Decode
`rxing-cli test_image.jpg decode --decode-multi`
