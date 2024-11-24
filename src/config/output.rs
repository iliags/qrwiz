use clap::{Args, ValueEnum};
use qrcode_generator::QrCodeEcc;

#[derive(Debug, Copy, Clone, PartialEq, ValueEnum)]
pub enum ImageFormat {
    PNG,
    SVG,
}

/// Re-export of QrCodeEcc from `qrcode-generator`
#[derive(Debug, Copy, Clone, PartialEq, ValueEnum)]
pub enum QrEcc {
    /// The QR Code can tolerate about  7% erroneous codewords.
    Low,
    /// The QR Code can tolerate about 15% erroneous codewords.
    Medium,
    /// The QR Code can tolerate about 25% erroneous codewords.
    Quartile,
    /// The QR Code can tolerate about 30% erroneous codewords.
    High,
}

impl From<QrEcc> for QrCodeEcc {
    fn from(ecc: QrEcc) -> Self {
        match ecc {
            QrEcc::Low => QrCodeEcc::Low,
            QrEcc::Medium => QrCodeEcc::Medium,
            QrEcc::Quartile => QrCodeEcc::Quartile,
            QrEcc::High => QrCodeEcc::High,
        }
    }
}

#[derive(Args, Debug)]
#[group(multiple = false)]
pub struct Output {
    /// The image size of the QR code
    // TODO: Write a custom validator to calculate power of 2
    #[arg(short, long, default_value = "128")]
    pub size: usize,

    /// The output format of the QR code
    #[arg(short, long, value_enum, default_value_t = ImageFormat::PNG)]
    pub format: ImageFormat,

    /// The error correction level of the QR code
    #[arg(short, long, value_enum, default_value_t = QrEcc::Low)]
    pub ecc: QrEcc,
}
