mod lib;
pub(crate)use lib::*;

use types::Context;
use cli::*;
use commands::*;
use aurras_primitives::Execute;
use std::process::exit;
use clap::Parser;

fn set_panic_hook() {
    #[cfg(not(debug_assertions))]
    std::panic::set_hook({
        Box::new(move |e| {
            eprintln!(
                "thread `{}` {}",
                std::thread::current().name().unwrap_or("<unnamed>"),
                e
            );
            eprintln!("stack backtrace: \n{:?}", backtrace::Backtrace::new());
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
            eprintln!("note: composer flags: {:?}\n", CLI::parse());
        })
    });
}

#[cfg(not(tarpaulin_include))]
pub fn handle_error<T>(res: types::Result<T>) -> T {
    match res {
        Ok(t) => t,
        Err(err) => {
            eprintln!("{err}");
            exit(err.code());
        }
    }
}

pub fn run_with_args(cli: CLI) -> types::Result<()> {
    if !cli.quiet() {

    }

    let context = handle_error(Context::new());
    match cli.command {
        Commands::Build { command } => command.execute(context)?,
    };

    Ok(())
}

fn main() {
    set_panic_hook();
    handle_error(run_with_args(CLI::parse()));
}