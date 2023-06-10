//! Interact with a remote source.
use super::cache::{Cache, Resource};
use super::header::Header;
use super::index::Index;
use crate::info::Info;
use crate::member::Member;

use anyhow::{anyhow, Result};
use bytes::Bytes;
use serde::Deserialize;

use std::fs;
use std::path::{Path, PathBuf};

const NAME_PLACEHOLDER: &str = "{name}";

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Patterns {
    pub(crate) info: String,
    pub(crate) grids: String,
}

impl Default for Patterns {
    fn default() -> Self {
        Patterns {
            info: "{name}/info.yaml".to_owned(),
            grids: "{name}.partons.lz4".to_owned(),
        }
    }
}

/// A remote registry.
///
/// It contains the information to connect to a remote data source, and the methods to fetch and
/// load the content.
#[derive(Deserialize, Debug, Clone)]
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

    async fn fetch(&self, url: &str, resource: &Resource) -> Result<Bytes> {
        // TODO: turn prints in logs
        let content = if let Some(cache) = self.cache.as_ref() {
            let location = cache.locate(resource)?;
            println!("location: {location:?}");

            if !location.exists() {
                let content = Self::download(url).await?;

                // TODO: move old to trash bin -> upgrade cache to struct
                fs::create_dir_all(
                    location
                        .parent()
                        .ok_or(anyhow!("Fail to access parent of '{location:?}'"))?,
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

    /// Fetch the source index.
    ///
    /// The index contains the information about the sets available in the remote.
    ///
    /// ```
    /// # use partons::configs::Configs;
    /// # use partons::data::index::Index;
    /// # use anyhow::Result;
    /// # use std::env;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// #     let mut path = env::current_dir()?;
    /// #     path.push("../partons.toml");
    ///       let configs = Configs::new(path)?;
    ///       let index: Index = configs.sources[0].index().await?;
    /// #     Ok(())
    /// # }
    /// ```
    pub async fn index(&self) -> Result<Index> {
        let content = self.fetch(&self.index, &Resource::Index).await?;

        std::str::from_utf8(&content)?
            .parse::<Index>()
            .map_err(|_| anyhow!("Failed to parse index"))
    }

    fn url(&self, path: &str) -> String {
        format!("{endpoint}{path}", endpoint = self.url).to_owned()
    }

    async fn load(&self, remote: &Path, resource: &Resource) -> Result<Bytes> {
        let url = self.url(
            remote
                .to_str()
                .ok_or(anyhow!("Invalid remote path {remote:?}"))?,
        );
        self.fetch(&url, resource).await
    }

    fn replace_name(pattern: &str, name: &str) -> PathBuf {
        PathBuf::from(pattern.replace(NAME_PLACEHOLDER, name))
    }

    /// Fetch set metadata.
    ///
    /// ```
    /// # use partons::configs::Configs;
    /// # use partons::info::Info;
    /// # use anyhow::Result;
    /// # use std::env;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// #     let mut path = env::current_dir()?;
    /// #     path.push("../partons.toml");
    ///       let configs = Configs::new(path)?;
    ///       let index = configs.sources[0].index().await?;
    ///       let entry = index.get("NNPDF40_nnlo_as_01180")?;
    ///       let info: Info = configs.sources[0].info(&entry).await?;
    /// #     Ok(())
    /// # }
    /// ```
    pub async fn info(&self, header: &Header) -> Result<Info> {
        let remote = Self::replace_name(&self.patterns.info, &header.name);
        let content = self
            .load(remote.as_path(), &Resource::Info(header.name.to_owned()))
            .await?;

        Info::load(content).map_err(|err| {
            anyhow!(
                "Failed to parse info file for {}:\n\t{:?}",
                header.identifier(),
                err
            )
        })
    }

    /// Fetch set member.
    ///
    /// ```
    /// # use partons::configs::Configs;
    /// # use partons::member::Member;
    /// # use anyhow::Result;
    /// # use std::env;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// #     let mut path = env::current_dir()?;
    /// #     path.push("../partons.toml");
    ///       let configs = Configs::new(path)?;
    ///       let index = configs.sources[0].index().await?;
    ///       let entry = index.get("NNPDF40_nnlo_as_01180")?;
    ///       let member: Member = configs.sources[0].member(&entry, 0).await?;
    /// #     Ok(())
    /// # }
    /// ```
    pub async fn member(&self, header: &Header, member: u32) -> Result<Member> {
        let remote = Self::replace_name(&self.patterns.grids, &header.name);

        let content = self
            .load(
                remote.as_path(),
                &Resource::Grid(header.name.to_owned(), member),
            )
            .await?;

        Member::load(content).map_err(|err| {
            anyhow!(
                "Failed to parse grid file for {}:\n\t{:?}",
                header.identifier(),
                err
            )
        })
    }
}
