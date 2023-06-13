use std::process::{ExitCode, Termination};

#[macro_use]
mod macros;

mod base;
mod cache;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
#[command(disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    command: base::Command,
}

fn main() -> ExitCode {
    let args = Cli::parse();

    match args.command.run() {
        Ok(code) => code,
        result @ Err(_) => result.report(),
    }
}
