use std::ops::{self, Deref};
use std::str::FromStr;
use std::vec;

use anyhow::{anyhow, Result};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::info::Info;
use crate::set::SetHeader;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Patterns {
    pub(crate) info: String,
    pub(crate) tarball: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Source {
    name: String,
    url: String,
    index: String,
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
    pub async fn fetch_index(&self) -> Result<Index> {
        let content = reqwest::get(&self.index).await?.text().await?;

        content
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
