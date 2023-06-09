/// Interact with a remote source.
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
    /// Register cache location.
    ///
    /// `data_path` is the path to the general `partons` data folder.
    ///
    /// ```
    /// # use partons::configs::Configs;
    /// # use partons::data::source::Source;
    /// # use anyhow::Result;
    /// # use std::env;
    /// #
    /// # fn main() -> Result<()> {
    /// #     let mut path = env::current_dir()?;
    /// #     path.push("../partons.toml");
    ///       let configs = Configs::new(path)?;
    ///       let mut source: Source = configs.sources[0].clone();
    ///       source.register_cache(configs.data_path()?);
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// See [`Configs`](crate::configs::Configs) to learn how to load them, and in particular
    /// [`Configs::new`](crate::configs::Configs::new).
    pub fn register_cache(&mut self, data_path: PathBuf) {
        self.cache = Some(Cache::new(&self.name, data_path));
    }

    // Download whatever remote resources to raw bytes
    async fn download(url: &str) -> Result<Bytes> {
        Ok(reqwest::get(url).await?.bytes().await?)
    }

    async fn fetch(&self, url: &str, path: &Path) -> Result<Bytes> {
        // TODO: turn prints in logs
        let content = if let Some(cache) = self.cache.as_ref() {
            let location = cache.locate(path)?;
            println!("{:#?}", location);

            if !location.exists() {
                let content = Self::download(url).await?;

                // TODO: move old to trash bin -> upgrade cache to struct
                fs::create_dir_all(
                    location
                        .parent()
                        .ok_or(anyhow!("Fail to access parent for '{location:?}'"))?,
                )?;

                fs::write(&location, &content)?;
                println!("'{url}' cached");

                content
            } else {
                let content: Bytes = fs::read(&location)?.into();
                println!("'{url}' loaded from cache");
                content
            }
        } else {
            Self::download(url).await?
        };

        Ok(content)
    }

    pub async fn index(&self) -> Result<Index> {
        let content = self.fetch(&self.index, Path::new(INDEX_NAME)).await?;

        std::str::from_utf8(&content)?
            .parse::<Index>()
            .map_err(|_| anyhow!("Failed to parse index"))
    }

    fn url(&self, path: &str) -> String {
        format!("{endpoint}{path}", endpoint = self.url).to_owned()
    }

    async fn load(&self, remote: &Path, header: &Header, local: &Path) -> Result<Bytes> {
        let url = self.url(
            remote
                .to_str()
                .ok_or(anyhow!("Invalid remote path in {}", header.identifier()))?,
        );
        self.fetch(&url, local).await
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
