use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

use crate::{
    info::Info,
    member::Grid,
    remote::{Patterns, Source},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Header<'head> {
    id: u32,
    name: String,
    number: u32,
    #[serde(skip)]
    source: Option<&'head Source>,
}

const NAME_PLACEHOLDER: &str = "{name}";

impl<'head> Header<'head> {
    pub fn new(id: u32, name: String, number: u32) -> Self {
        Self {
            id,
            name,
            number,
            source: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn identifier(&self) -> String {
        format!("{}:'{}'", self.name, self.id)
    }

    pub fn source(&self) -> Result<&Source> {
        self.source.ok_or(anyhow!(""))
    }

    pub fn patterns(&self) -> Result<&Patterns> {
        Ok(&self.source()?.patterns)
    }

    pub async fn load(&self, remote: &Path, local: &Path, cache: Option<&Path>) -> Result<Bytes> {
        let source = self.source()?;
        let url = source.url(
            remote
                .to_str()
                .ok_or(anyhow!("Invalid remote path in {}", self.identifier()))?,
        );
        let location = source.fetch(&url, local, cache).await?;

        Ok(fs::read(&location)?.into())
    }

    pub async fn info(&self, cache: Option<&Path>) -> Result<Info> {
        let pattern = &self.patterns()?.info;
        let path = PathBuf::from(pattern.replace(NAME_PLACEHOLDER, &self.name));
        let content = self.load(path.as_path(), path.as_path(), cache).await?;

        serde_yaml::from_slice(&content).map_err(|err| {
            anyhow!(
                "Failed to parse info file for {}:\n\t{:?}",
                self.identifier(),
                err
            )
        })
    }

    pub async fn grid(&self, member: u32, cache: Option<&Path>) -> Result<Grid> {
        let pattern = &self.patterns()?.grids;
        let remote = PathBuf::from(pattern.replace(NAME_PLACEHOLDER, &self.name));
        let mut local = PathBuf::from(&self.name);
        local.push(format!("{}.member.lz4", member));

        let content = self.load(local.as_path(), remote.as_path(), cache).await?;

        serde_yaml::from_slice(&content).map_err(|err| {
            anyhow!(
                "Failed to parse grid file for {}:\n\t{:?}",
                self.identifier(),
                err
            )
        })
    }
}
