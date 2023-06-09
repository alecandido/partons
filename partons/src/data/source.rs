use super::cache::Cache;
use super::header::{self, Header};
use super::index::Index;
use super::lhapdf::info::Info;
use crate::member::Grid;

use anyhow::{anyhow, Result};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Patterns {
    pub(crate) info: String,
    pub(crate) grids: String,
}

impl Default for Patterns {
    fn default() -> Self {
        Patterns {
            info: "{name}/info.yaml".to_owned(),
            grids: "{name}.lz4".to_owned(),
        }
    }
}

const INDEX_NAME: &str = "index.csv";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Source {
    name: String,
    url: String,
    index: String,
    #[serde(default)]
    pub(crate) patterns: Patterns,
    #[serde(skip)]
    cache: Option<Cache>,
}

impl Source {
    fn url(&self, path: &str) -> String {
        format!("{endpoint}{path}", endpoint = self.url).to_owned()
    }

    pub fn register_cache(&mut self, data_path: PathBuf) {
        self.cache = Some(Cache::new(&self.name, data_path));
    }

    async fn fetch(&self, url: &str, path: &Path) -> Result<PathBuf> {
        // TODO: turn prints in logs
        let location = self
            .cache
            .as_ref()
            .ok_or(anyhow!("Cache not present"))?
            .locate(path)?;

        if !location.exists() {
            let content = reqwest::get(url).await?.bytes().await?;
            println!("{:#?}", location);
            // TODO: move old to trash bin -> upgrade cache to struct
            fs::create_dir_all(
                location
                    .parent()
                    .ok_or(anyhow!("Fail to access parent for '{location:?}'"))?,
            )?;

            // Here, in principle, it would be possible to directly return content, without
            // reloading it from disk. However, this would uselessly complicate the workflow, since
            // this function would be in charge of loading as well, and the content should be
            // propagated.
            // Anyhow: if the content has been remotely fetched and dumped, reading it is not the
            // most expensive operation.
            fs::write(&location, &content)?;
        } else {
            println!("'{url}' loaded from cache");
        };

        Ok(location)
    }

    pub async fn index(&self) -> Result<Index> {
        let location = self.fetch(&self.index, Path::new(INDEX_NAME)).await?;
        let content: Bytes = fs::read(&location)?.into();

        std::str::from_utf8(&content)?
            .parse::<Index>()
            .map_err(|_| anyhow!("Failed to parse index"))
    }

    async fn load(&self, remote: &Path, header: &Header, local: &Path) -> Result<Bytes> {
        let url = self.url(
            remote
                .to_str()
                .ok_or(anyhow!("Invalid remote path in {}", header.identifier()))?,
        );
        let location = self.fetch(&url, local).await?;

        Ok(fs::read(&location)?.into())
    }

    async fn info(&self, header: &Header) -> Result<Info> {
        let pattern = &self.patterns.info;
        let path = PathBuf::from(pattern.replace(header::NAME_PLACEHOLDER, &self.name));
        let content = self.load(path.as_path(), header, path.as_path()).await?;

        serde_yaml::from_slice(&content).map_err(|err| {
            anyhow!(
                "Failed to parse info file for {}:\n\t{:?}",
                header.identifier(),
                err
            )
        })
    }

    async fn grid(&self, member: u32, header: &Header, cache: Option<&Path>) -> Result<Grid> {
        let pattern = &self.patterns.grids;
        let remote = PathBuf::from(pattern.replace(header::NAME_PLACEHOLDER, &self.name));
        let mut local = PathBuf::from(&self.name);
        local.push(format!("{}.member.lz4", member));

        let content = self.load(local.as_path(), header, remote.as_path()).await?;

        serde_yaml::from_slice(&content).map_err(|err| {
            anyhow!(
                "Failed to parse grid file for {}:\n\t{:?}",
                header.identifier(),
                err
            )
        })
    }
}
