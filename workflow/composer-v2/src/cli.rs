use clap::StructOpt;
use std::path::PathBuf;
use crate::commands::Commands;

/// CLI Arguments entry point - includes global parameters and subcommands
#[derive(StructOpt, Debug)]
#[structopt(name = "composer", author = "The HugoByte Team <hello@hugobyte.com>")]
pub struct CLI {
    #[structopt(short, global = true, help = "Print additional information for debugging")]
    pub debug: bool,

    #[structopt(short, global = true, help = "Suppress CLI output")]
    pub quiet: bool,

    #[structopt(subcommand)]
    pub command: Commands,

    #[structopt(
        long,
        global = true,
        help = "Optional path to output workflow wasm",
        parse(from_os_str)
    )]
    pub output: Option<PathBuf>,
}

impl CLI {
    pub fn quiet(&self) -> bool {
        self.quiet
    }
}