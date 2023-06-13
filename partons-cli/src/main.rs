use std::process::{ExitCode, Termination};

mod cache;
mod subcommand;

use clap::Parser;

#[derive(Parser)]
#[command(name = "partons")]
#[command(author, version, about)]
#[command(disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    command: subcommand::Command,
}

fn main() -> ExitCode {
    let args = Cli::parse();

    match args.command.run() {
        Ok(code) => code,
        result @ Err(_) => result.report(),
    }
}
