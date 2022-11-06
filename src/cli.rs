use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub config_file: String,
}