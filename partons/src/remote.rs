use std::str::FromStr;

use anyhow::{anyhow, Result};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    name: String,
    url: String,
    index: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetHeader {
    id: u32,
    name: String,
    number: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    sets: Vec<SetHeader>,
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

            index.sets.push(SetHeader {
                id: id.parse().map_err(|_| ParseIndexError)?,
                name: name.to_owned(),
                number: number.parse().map_err(|_| ParseIndexError)?,
            })
        }

        Ok(index)
    }
}

impl Source {
    pub async fn fetch_index(&self) -> Result<Index> {
        let content = reqwest::get(&self.index).await?.text().await?;

        content
            .parse::<Index>()
            .map_err(|_| anyhow!("Failed to parse index"))
    }
}
