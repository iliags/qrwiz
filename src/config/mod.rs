use clap::Parser;
use output::Output;
//use std::{ffi::OsStr, path::PathBuf};
use url::Url;

pub mod output;

/// QR code generator configuration
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// List of URLs to convert
    #[clap(short, long, value_parser, num_args = 0.., value_delimiter = ' ', value_parser=validate_url)]
    pub urls: Vec<Url>,

    /*
    /// The configuration file to use
    #[arg(short, long, value_parser=validate_toml)]
    pub config_file: Option<String>,
     */
    /// The output configuration
    #[command(flatten)]
    pub output: Output,
}

/*
fn validate_toml(value: &str) -> Result<String, String> {
    let path = PathBuf::from(value);
    if path.extension() != Some(OsStr::new("toml")) {
        return Err("only TOML files are supported".to_string());
    }

    Ok(value.to_string())
}
*/

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
