//! Remote index
use super::header::Header;

use anyhow::{bail, Result};
use itertools::Itertools;
use regex::Regex;
use serde::{Deserialize, Serialize};

use std::ops::{self, Deref};
use std::str::FromStr;
use std::vec;

/// Describe content of a remote source.
#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    sets: Vec<Header>,
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
