//! Configs operations.
use std::process::ExitCode;

use anyhow::Result;
use clap::{Args, Subcommand};

use partons::configs::Configs;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct ConfigsArgs {
    #[command(subcommand)]
    command: Option<ConfigsCommands>,
}

impl ConfigsArgs {
    pub(crate) fn run(self) -> Result<ExitCode> {
        match self.command {
            None => ListArgs::default().run(),
            Some(command) => command.run(),
        }
    }
}

#[derive(Debug, Subcommand)]
enum ConfigsCommands {
    List(ListArgs),
}

impl ConfigsCommands {
    fn run(self) -> Result<ExitCode> {
        run!(self; List)
    }
}

#[derive(Debug, Args)]
struct ListArgs {}

impl Default for ListArgs {
    fn default() -> Self {
        ListArgs {}
    }
}

impl ListArgs {
    fn run(self) -> Result<ExitCode> {
        let configs = serde_json::to_string_pretty(&Configs::load()?)?;
        println!("{configs}");
        Ok(ExitCode::SUCCESS)
    }
}
