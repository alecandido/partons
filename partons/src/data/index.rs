//! Remote index
use super::header::Header;

use anyhow::Result;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use std::ops::{self, Deref};
use std::str::FromStr;
use std::vec;

/// Describe content of a remote source.
#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    pub sets: Vec<Header>,
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
