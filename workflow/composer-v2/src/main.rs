#![allow(special_module_name)]
mod lib;
pub(crate) use lib::*;

use clap::Parser;
use cli::*;
use commands::*;
use composer_primitives::{Execute, Result};
use std::process::exit;
use types::Context;

fn set_panic_hook() {
    #[cfg(not(debug_assertions))]
    std::panic::set_hook({
        Box::new(move |e| {
            eprintln!(
                "thread `{}` {}",
                std::thread::current().name().unwrap_or("<unnamed>"),
                e
            );
            eprintln!(
                "stack backtrace: \n{:?}",
                std::backtrace::Backtrace::capture()
            );
            eprintln!("error: internal composer error: unexpected panic\n");
            eprintln!("note: the composer unexpectedly panicked. this is a bug.\n");
            eprintln!(
                "note: {} {} running on {} {}\n",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                sys_info::os_type().unwrap_or_else(|e| e.to_string()),
                sys_info::os_release().unwrap_or_else(|e| e.to_string()),
            );
            eprintln!(
                "note: composer args: {}\n",
                std::env::args().collect::<Vec<_>>().join(" ")
            );
            eprintln!("note: composer flags: {:?}\n", Cli::parse());
        })
    });
}

#[cfg(not(tarpaulin_include))]
pub fn handle_error<T>(res: Result<T>) -> T {
    match res {
        Ok(t) => t,
        Err(err) => {
            eprintln!("{:?}", err);
            exit(err.code());
        }
    }
}

pub fn run_with_args(cli: Cli) -> Result<()> {
    let mut context = handle_error(Context::new());
    if !cli.quiet() {
        context.quiet();
    }

    match cli.command {
        Commands::Build { command } => command.execute(context)?,
        Commands::Create { command } => command.execute()?,
        Commands::Validate { command } => command.execute(context)?,
    };

    Ok(())
}

fn main() {
    set_panic_hook();
    handle_error(run_with_args(Cli::parse()));
}
