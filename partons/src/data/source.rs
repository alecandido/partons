//! Interact with a remote source.
use super::cache::{Cache, Resource};
use super::header::Header;
use super::index::Index;
use super::lhapdf;
use crate::info::Info;

use anyhow::{anyhow, Result};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use std::path::{Path, PathBuf};

const NAME_PLACEHOLDER: &str = "{name}";

#[derive(Serialize, Deserialize, Debug, Clone)]
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

/// Error during data conversion
#[derive(Error, Debug)]
pub enum ConversionError {
    /// Missing field from original value
    #[error("Missing field {0}")]
    MissingField(String),
    /// Type mismatched
    #[error("Missing field {0}")]
    FieldType(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Format {
    Native,
    Lhapdf,
}

impl Default for Format {
    fn default() -> Self {
        Format::Native
    }
}

impl Format {
    fn convert(&self, content: Bytes, resource: &Resource) -> Result<Bytes> {
        match self {
            Self::Native => Ok(content),
            Self::Lhapdf => lhapdf::convert(content, resource),
        }
    }
}

/// A remote registry.
///
/// It contains the information to connect to a remote data source, and the methods to fetch and
/// load the content.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Source {
    pub(crate) name: String,
    url: String,
    index: String,
    #[serde(default)]
    format: Format,
    #[serde(default)]
    pub(crate) patterns: Patterns,
    /// The data cache
    ///
    /// Since it should be loaded separately from configurations, during configs deserialization is
    /// set to `None`.
    // TODO: consider to store source configs in a separate struct, and deserialize that.
    #[serde(skip)]
    pub cache_: Option<Cache>,
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
        self.cache_ = Some(Cache::new(&self.name, data_path));
    }

    fn cache(&self) -> Result<&Cache> {
        self.cache_.as_ref().ok_or(anyhow!("Cache not registered."))
    }

    // Download whatever remote resources to raw bytes
    async fn download(url: &str) -> Result<Bytes> {
        Ok(reqwest::get(url).await?.bytes().await?)
    }

    async fn converted(&self, url: &str, resource: &Resource) -> Result<Bytes> {
        let raw = resource.raw_path();
        let cache = self.cache()?;

        let content = if !raw.exists() {
            let content = Self::download(url).await?;
            // cache the raw contnet
            cache.write(resource, &content)?;

            let content = cache.unpack(resource, content)?;
            content
        } else {
            cache.read(resource)?
        };

        self.format.convert(content, resource)
    }

    // Download whatever remote resources to raw bytes
    async fn fetch(&self, url: &str, resource: &Resource) -> Result<Bytes> {
        // TODO: turn prints in logs
        println!("Fetching content from {url}");
        let cache = self.cache()?;

        let content = if !cache.exists(resource) {
            println!("caching resource '{resource}'");
            let content = self.converted(url, resource).await?;

            cache.write(resource, &content)?;

            content
        } else {
            println!("loading from location '{resource}'");
            cache.read(resource)?
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

    /// `remote` is the URL path on the remote source.
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
    pub async fn set(&self, header: &Header) -> Result<()> {
        let remote = Self::replace_name(&self.patterns.grids, &header.name);

        self.load(remote.as_path(), &Resource::Set(header.name.to_owned()))
            .await?;

        Ok(())
    }
}
