use crate::commands::Commands;
use clap::StructOpt;

/// CLI Arguments entry point - includes global parameters and subcommands
#[derive(StructOpt, Debug)]
#[structopt(name = "composer", author = "The HugoByte Team <hello@hugobyte.com>")]
pub struct CLI {
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

impl CLI {
    pub fn quiet(&self) -> bool {
        self.quiet
    }
}
