use super::index::Index;

use anyhow::{anyhow, Result};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Patterns {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Source {
    name: String,
    url: String,
    index: String,
    #[serde(default)]
    pub(crate) patterns: Patterns,
}

// Create a struct to be able to implement FromStr, with a type alias it would be impossible

impl Source {
    pub fn url(&self, path: &str) -> String {
        format!("{endpoint}{path}", endpoint = self.url).to_owned()
    }

    pub fn in_cache(&self, path: &Path, cache: Option<&Path>) -> Result<PathBuf> {
        let mut buf = cache.ok_or(anyhow!("Cache not present"))?.to_owned();
        buf.push(&self.name);
        buf.push(path);

        Ok(buf)
    }

    pub async fn fetch(&self, url: &str, path: &Path, cache: Option<&Path>) -> Result<PathBuf> {
        // TODO: turn prints in logs
        let location = self.in_cache(path, cache)?;

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

    pub async fn index(&self, cache: Option<&Path>) -> Result<Index> {
        let location = self
            .fetch(&self.index, Path::new("index.csv"), cache)
            .await?;
        let content: Bytes = fs::read(&location)?.into();

        std::str::from_utf8(&content)?
            .parse::<Index>()
            .map_err(|_| anyhow!("Failed to parse index"))
    }
}
