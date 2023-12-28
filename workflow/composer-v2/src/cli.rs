use crate::commands::Commands;
use clap::StructOpt;

/// Cli Arguments entry point - includes global parameters and subcommands
#[derive(StructOpt, Debug)]
#[structopt(
    name = "composer",
    author = "The HugoByte Team <hello@hugobyte.com>",
    version = "0.0.1"
)]
pub struct Cli {
    #[structopt(
        short,
        global = true,
        help = "Print additional information for debugging"
    )]
    pub debug: bool,

    #[structopt(short, global = true, help = "Suppress CLI output")]
    pub quiet: bool,

    #[structopt(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn quiet(&self) -> bool {
        self.quiet
    }
}
