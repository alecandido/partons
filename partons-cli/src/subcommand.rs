//! Subcommand commons.
use std::process::ExitCode;

use anyhow::Result;
use clap::Subcommand;

use super::cache;

#[derive(Subcommand)]
pub(crate) enum Command {
    Cache(cache::Subcommand),
}

impl Command {
    pub(crate) fn run(self) -> Result<ExitCode> {
        Ok(ExitCode::SUCCESS)
    }
}
