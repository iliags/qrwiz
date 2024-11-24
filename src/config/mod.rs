use clap::Parser;
use output::Output;
use std::{ffi::OsStr, path::PathBuf};
use url::Url;

pub mod input;
pub mod output;

/// QR code generator configuration
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// List of files to convert
    //#[clap(short, long, value_parser, num_args = 0.., value_delimiter = ' ')]
    //pub files: Vec<String>,

    /// List of URLs to convert
    #[clap(short, long, value_parser, num_args = 0.., value_delimiter = ' ')]
    pub urls: Vec<Url>,

    // A list of files inside a CSV to convert to QR codes
    //#[arg(short = 'l', long)]
    //pub file_list: Option<String>,
    /// The configuration file to use
    #[arg(short, long, value_parser=valid_toml)]
    pub config_file: Option<String>,

    #[command(flatten)]
    pub output: Output,
}

fn valid_toml(value: &str) -> Result<String, String> {
    let path = PathBuf::from(value);
    if path.extension() != Some(OsStr::new("toml")) {
        return Err("only TOML files are supported".to_string());
    }

    Ok(value.to_string())
}
