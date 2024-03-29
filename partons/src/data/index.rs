//! Remote index
use std::ops::{self, Deref};
use std::str::FromStr;
use std::vec;

use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use regex::Regex;
use serde::{Deserialize, Serialize};

use super::header::Header;
use super::resource::Data;
use super::source::Source;

/// Describe content of a remote source.
#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    sets: Vec<Header>,
}

/// Error during parsing of index file
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
impl ops::Index<usize> for Index {
    type Output = Header;

    fn index(&self, index: usize) -> &Self::Output {
        &self.sets[index]
    }
}

// Iterate underlying vector
impl Deref for Index {
    type Target = [Header];

    fn deref(&self) -> &Self::Target {
        self.sets.deref()
    }
}

impl IntoIterator for Index {
    type Item = Header;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.sets.into_iter()
    }
}

impl Index {
    /// Retrieve set header from name or pattern.
    ///
    /// Regular expressions are supported through [`regex`].
    pub fn get(&self, name: &str) -> Result<Header> {
        let full_match = "^".to_owned() + name + "$";
        let re = Regex::new(&full_match)?;
        let found: Vec<_> = self
            .sets
            .iter()
            .filter(|header| re.is_match(&header.name))
            .collect();
        match found.len() {
            0 => bail!("No set matching {name}."),
            1 => Ok(found[0].to_owned()),
            n => bail!("{n} sets found matching {name}"),
        }
    }
}

impl Source {
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
    /// # fn main() -> Result<()> {
    /// #     let mut path = env::current_dir()?;
    /// #     path.push("../partons.toml");
    ///       let configs = Configs::new(path)?;
    ///       let mut source = configs.sources[0].clone();
    ///       source.register_cache(configs.data_path()?);
    ///       let index: Index = source.index()?;
    /// #     Ok(())
    /// # }
    /// ```
    pub fn index(&self) -> Result<Index> {
        let content = self.fetch(&self.index, Data::Index)?;

        std::str::from_utf8(&content)?
            .parse::<Index>()
            .map_err(|_| anyhow!("Failed to parse index"))
    }
}
