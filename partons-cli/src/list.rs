/// List content
use std::process::ExitCode;

use anyhow::Result;
use clap::{Args, Subcommand};

use partons::configs::Configs;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct ListArgs {
    #[command(subcommand)]
    command: Option<ListCommands>,
}

impl ListArgs {
    pub(crate) fn run(self) -> Result<ExitCode> {
        match self.command {
            None => LocalArgs::default().run(),
            Some(command) => command.run(),
        }
    }
}

#[derive(Debug, Subcommand)]
enum ListCommands {
    Local(LocalArgs),
    Remote(RemoteArgs),
}

impl ListCommands {
    fn run(self) -> Result<ExitCode> {
        run!(self; Local, Remote)
    }
}

#[derive(Debug, Args)]
struct LocalArgs {}

impl Default for LocalArgs {
    fn default() -> Self {
        LocalArgs {}
    }
}

impl LocalArgs {
    fn run(self) -> Result<ExitCode> {
        let configs = Configs::load()?;
        let path = configs.data_path()?;
        let mut sources = configs.sources;

        let mut sets = Vec::new();
        for source in sources.iter_mut() {
            source.register_cache(path.clone());
            sets.extend(source.cache.clone().unwrap().sets()?)
        }

        sets.sort();
        for set in sets.iter() {
            println!("{set}");
        }
        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Debug, Args)]
struct RemoteArgs {}

impl RemoteArgs {
    fn run(self) -> Result<ExitCode> {
        println!("Remote content");
        Ok(ExitCode::SUCCESS)
    }
}
