//! Interact with a remote source.
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

use super::cache::file::Cache;
use super::format::Format;
use super::resource::{Data, Resource, State};

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

/// A remote registry.
///
/// It contains the information to connect to a remote data source, and the methods to fetch and
/// load the content.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Source {
    pub(crate) name: String,
    url: String,
    pub(crate) index: String,
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
    pub cache: Option<Cache>,
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

    fn cache(&self) -> Result<&Cache> {
        self.cache.as_ref().ok_or(anyhow!("Cache not registered."))
    }

    // Download whatever remote resources to raw bytes
    async fn download(url: &str) -> Result<Bytes> {
        Ok(reqwest::get(url).await?.bytes().await?)
    }

    async fn converted(&self, url: &str, data: Data) -> Result<Bytes> {
        let resource = Resource {
            data,
            state: State::Original,
        };
        let cache = self.cache()?;

        let content = if !cache.exists(&resource) {
            let content = Self::download(url).await?;
            // cache the raw contnet
            cache.write(&resource, &content)?;

            let content = cache.unpack(&resource, &self.format, content)?;
            content
        } else {
            cache.read(&resource)?
        };

        self.format.convert(content, &resource.data)
    }

    // Download whatever remote resources to raw bytes
    pub(crate) async fn fetch(&self, url: &str, data: Data) -> Result<Bytes> {
        // TODO: turn prints in logs
        println!("Fetching content from {url}");
        let cache = self.cache()?;
        let resource = Resource {
            data,
            state: State::Regular,
        };

        let content = if !cache.exists(&resource) {
            println!("caching resource '{resource}'");
            let content = self.converted(url, resource.data.clone()).await?;

            cache.write(&resource, &content)?;

            content
        } else {
            println!("loading from location '{resource}'");
            cache.read(&resource)?
        };

        Ok(content)
    }

    fn url(&self, path: &str) -> String {
        format!("{endpoint}{path}", endpoint = self.url).to_owned()
    }

    /// `remote` is the URL path on the remote source.
    pub(crate) async fn load(&self, remote: &Path, data: Data) -> Result<Bytes> {
        let url = self.url(
            remote
                .to_str()
                .ok_or(anyhow!("Invalid remote path {remote:?}"))?,
        );
        self.fetch(&url, data).await
    }

    pub(crate) fn replace_name(pattern: &str, name: &str) -> PathBuf {
        PathBuf::from(pattern.replace(NAME_PLACEHOLDER, name))
    }
}
