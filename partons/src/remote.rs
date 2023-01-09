use crate::set::Header;

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
pub struct Index<'index> {
    pub sets: Vec<Header<'index>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseIndexError;

impl<'index> FromStr for Index<'index> {
    type Err = ParseIndexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut index = Self { sets: vec![] };
        for line in s.lines() {
            let (id, name, number) = line
                .split_whitespace()
                .next_tuple()
                .ok_or(ParseIndexError)?;

            index.sets.push(Header::new(
                id.parse().map_err(|_| ParseIndexError)?,
                name.to_owned(),
                number.parse().map_err(|_| ParseIndexError)?,
            ))
        }

        Ok(index)
    }
}

// Index underlying vector
impl<'index> ops::Index<usize> for Index<'index> {
    type Output = Header<'index>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.sets[index]
    }
}

// Iterate underlying vector
impl<'index> Deref for Index<'index> {
    type Target = [Header<'index>];

    fn deref(&self) -> &Self::Target {
        self.sets.deref()
    }
}

impl<'index> IntoIterator for Index<'index> {
    type Item = Header<'index>;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.sets.into_iter()
    }
}

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

            // Here in principle it would be possible to directly return content, without reloading
            // it from disk. However, this would uselessly complicate the workflow, since this
            // function would be in charge of loading as well, and the content should be
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
