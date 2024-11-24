use clap::{Args, Parser, Subcommand, ValueEnum};

pub mod input;

/// QR code generator configuration
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {}
