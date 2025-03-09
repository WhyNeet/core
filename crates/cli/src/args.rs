use clap::Parser;

#[derive(Debug, Parser)]
pub struct CliArgs {
    pub url: String,
}
