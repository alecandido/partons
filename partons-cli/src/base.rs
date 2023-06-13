//! Subcommand commons.
use std::process::ExitCode;

use anyhow::Result;
use clap::Subcommand;

use super::cache::CacheArgs;

#[derive(Subcommand)]
pub(crate) enum Command {
    Cache(CacheArgs),
}

impl Command {
    pub(crate) fn run(self) -> Result<ExitCode> {
        run!(self; Cache)
    }
}
