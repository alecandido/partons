use crate::info::Info;
use crate::set::SetHeader;

use anyhow::{anyhow, Result};
use bytes::Bytes;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use std::ops::{self, Deref};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fs, vec};

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
#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    pub sets: Vec<SetHeader>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseIndexError;

impl FromStr for Index {
    type Err = ParseIndexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut index = Self { sets: vec![] };
        for line in s.lines() {
            let (id, name, number) = line
                .split_whitespace()
                .next_tuple()
                .ok_or(ParseIndexError)?;

            index.sets.push(SetHeader::new(
                id.parse().map_err(|_| ParseIndexError)?,
                name.to_owned(),
                number.parse().map_err(|_| ParseIndexError)?,
            ))
        }

        Ok(index)
    }
}

// Index underlying vector
impl ops::Index<usize> for Index {
    type Output = SetHeader;

    fn index(&self, index: usize) -> &Self::Output {
        &self.sets[index]
    }
}

// Iterate underlying vector
impl Deref for Index {
    type Target = [SetHeader];

    fn deref(&self) -> &Self::Target {
        self.sets.deref()
    }
}

impl IntoIterator for Index {
    type Item = SetHeader;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.sets.into_iter()
    }
}

impl Source {
    pub fn in_cache(&self, path: &Path, cache: Option<&Path>) -> Result<PathBuf> {
        let mut buf = cache.ok_or(anyhow!("Cache not present"))?.to_owned();
        buf.push(&self.name);
        buf.push(path);

        Ok(buf)
    }

    pub async fn fetch(&self, url: &str, path: &Path, cache: Option<&Path>) -> Result<Bytes> {
        // TODO: turn prints in logs
        let location = self.in_cache(path, cache).ok();

        // let content = if let Some(ref locpath) = location && locpath.exists() {
        let content = if location.is_some() && location.clone().unwrap().exists() {
            let content = fs::read(&location.unwrap())?.into();
            println!("'{url}' loaded from cache");
            content
        } else {
            let content = reqwest::get(url).await?.bytes().await?;
            println!("{:#?}", location);
            if let Some(ref locpath) = location {
                // TODO: move old to trash bin -> upgrade cache to struct
                fs::create_dir_all(
                    locpath
                        .parent()
                        .ok_or(anyhow!("Fail to access parent for '{locpath:?}'"))?,
                )?;
                fs::write(locpath, &content)?;
            } else {
                println!("'{url}' downloaded, but not cached")
            }

            content
        };

        Ok(content)
    }

    pub async fn fetch_index(&self, cache: Option<&Path>) -> Result<Index> {
        let content = self
            .fetch(&self.index, Path::new("index.csv"), cache)
            .await?;

        std::str::from_utf8(&content)?
            .parse::<Index>()
            .map_err(|_| anyhow!("Failed to parse index"))
    }

    pub async fn fetch_info(&self, path: &str) -> Result<Info> {
        let url = format!("{endpoint}{path}", endpoint = self.url);
        let content = reqwest::get(&url).await?.text().await?;

        serde_yaml::from_str(&content)
            .map_err(|e| anyhow!("Failed to parse info file for '{}':\n\t{:?}", path, e))
    }
}
