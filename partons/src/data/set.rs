use std::collections::HashMap;

use anyhow::{anyhow, Result};

use super::header::Header;
use super::resource::Data;
use super::source::{runtime, Source};
use crate::member::Member;
use crate::set::Set;

impl Source {
    /// Fetch set member.
    pub async fn set(&self, header: &Header) -> Result<Set> {
        let remote = Self::replace_name(&self.patterns.grids, &header.name);

        self.load(remote.as_path(), Data::Set(header.name.to_owned()))
            .await?;

        Ok(Set {
            source: self.clone(),
            header: header.clone(),
            info: None,
            members: HashMap::new(),
        })
    }

    /// Fetch member asynchronously.
    pub async fn member(&self, header: &Header, num: u32) -> Result<Member> {
        let remote = Self::replace_name(&self.patterns.grids, &header.name);

        let content = self
            .load(remote.as_path(), Data::Member(header.name.to_owned(), num))
            .await?;

        Member::load(content).map_err(|err| {
            anyhow!(
                "Failed to parse member file for {}-{}:\n\t{:?}",
                header.identifier(),
                num,
                err
            )
        })
    }
}

impl Member {
    /// Fetch member synchronously.
    pub fn fetch(source: &Source, header: &Header, num: u32) -> Result<Self> {
        runtime().block_on(source.member(header, num))
    }
}
