use clap::{Args, Parser, ValueEnum};
use qrcode_generator::QrCodeEcc;
use url::Url;

#[derive(Debug, Copy, Clone, PartialEq, ValueEnum)]
pub enum ImageFormat {
    PNG,
    SVG,
}

impl ImageFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            ImageFormat::PNG => "png",
            ImageFormat::SVG => "svg",
        }
    }
}

/// Re-export of QrCodeEcc from `qrcode-generator` for use with `ValueEnum`
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

/// QR code generator configuration
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// List of URLs to convert
    #[clap(short, long, value_parser, num_args = 0.., value_delimiter = ' ', value_parser=validate_url)]
    pub urls: Vec<Url>,

    /// The output configuration
    #[command(flatten)]
    pub output: Output,
}

fn validate_url(value: &str) -> Result<Url, String> {
    // TODO: Validate the URL instead of trusting user input
    if value.starts_with("http://") || value.starts_with("https://") {
        match Url::parse(value) {
            Ok(url) if url.scheme() == "http" || url.scheme() == "https" => return Ok(url),
            _ => return Err("URL is not valid".to_string()),
        }
    }

    let prefixed_value = "https://".to_string() + value;
    match Url::parse(&prefixed_value) {
        Ok(url) => Ok(url),
        Err(e) => Err(e.to_string()),
    }
}

#[derive(Args, Debug)]
#[group(multiple = true)]
pub struct Output {
    /// The image size of the QR code
    #[arg(short, long, default_value = "128")]
    pub size: usize,

    /// The output format of the QR code
    #[arg(short, long, value_enum, default_value_t = ImageFormat::SVG)]
    pub image_format: ImageFormat,

    /// The error correction level of the QR code
    #[arg(short, long, value_enum, default_value_t = QrEcc::Low)]
    pub ecc: QrEcc,

    /// The output file name of the QR code. If not provided, the input file name will be used.
    #[arg(short, long)]
    pub name: Option<String>,

    /// The output directory of the QR code. If not provided, the input file name will be used.
    #[arg(short, long)]
    pub dir: Option<String>,
}
