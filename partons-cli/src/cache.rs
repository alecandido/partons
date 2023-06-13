//! Cache operations.
use std::process::ExitCode;

use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct CacheArgs {
    #[command(subcommand)]
    command: Option<CacheCommands>,
}

impl CacheArgs {
    pub(crate) fn run(self) -> Result<ExitCode> {
        match self.command {
            None => InfoArgs::default().run(),
            Some(command) => command.run(),
        }
    }
}

#[derive(Debug, Subcommand)]
enum CacheCommands {
    Drop(DropArgs),
    Info(InfoArgs),
}

impl CacheCommands {
    fn run(self) -> Result<ExitCode> {
        run!(self; Drop, Info)
    }
}

#[derive(Debug, Args)]
struct DropArgs {}

impl DropArgs {
    fn run(self) -> Result<ExitCode> {
        println!("drop my cache");
        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Debug, Args)]
struct InfoArgs {}

impl Default for InfoArgs {
    fn default() -> Self {
        InfoArgs {}
    }
}

impl InfoArgs {
    fn run(self) -> Result<ExitCode> {
        println!("a lot of information");
        Ok(ExitCode::SUCCESS)
    }
}
