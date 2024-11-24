use std::{ffi::OsStr, path::PathBuf};

use clap::{
    error::{ContextKind, ContextValue, ErrorKind},
    Args, Parser, Subcommand, ValueEnum,
};

pub mod input;

/// QR code generator configuration
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// List of files to generate QR codes for
    #[clap(short, long, value_parser, num_args = 0.., value_delimiter = ' ')]
    pub files: Vec<String>,

    /// A list of files inside a CSV to convert to QR codes
    #[arg(short = 'l', long)]
    pub file_list: Option<String>,

    /// The configuration file to use
    //#[arg(short, long, value_parser=clap::builder::TryMapValueParser::new(ConfigFile, verify_config))]
    #[arg(short, long, value_parser=CustomValueParser)]
    pub config_file: Option<String>,
}

#[derive(Clone)]
struct CustomValueParser;

impl clap::builder::TypedValueParser for CustomValueParser {
    type Value = String;
    fn parse_ref(
        &self,
        _: &clap::Command,
        _: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let path = PathBuf::from(value);
        if path.extension() != Some(OsStr::new("toml")) {
            //return Err("only Rust files are supported");
            let mut error = clap::Error::new(ErrorKind::ValueValidation);
            let value = value.to_str().unwrap();

            error.insert(
                ContextKind::InvalidArg,
                ContextValue::String("--config_file".to_string()),
            );

            error.insert(
                ContextKind::InvalidValue,
                ContextValue::String(value.to_string()),
            );
            error.insert(
                ContextKind::SuggestedValue,
                ContextValue::String("toml".to_string()),
            );
            return Err(error);
        }

        let value = value.to_str().unwrap();
        Ok(value.to_string())
    }
}
