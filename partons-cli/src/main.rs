use std::process::{ExitCode, Termination};

use anyhow::Result;
use clap::{Parser, Subcommand};

#[macro_use]
mod macros;

mod cache;
mod configs;
mod list;

#[derive(Parser)]
#[command(name = "partons")]
#[command(author, version, about)]
#[command(disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    Cache(cache::CacheArgs),
    Configs(configs::ConfigsArgs),
    List(list::ListArgs),
}

impl Command {
    pub(crate) fn run(self) -> Result<ExitCode> {
        run!(self; Cache, Configs, List)
    }
}

fn main() -> ExitCode {
    let args = Cli::parse();

    match args.command.run() {
        Ok(code) => code,
        result @ Err(_) => result.report(),
    }
}
