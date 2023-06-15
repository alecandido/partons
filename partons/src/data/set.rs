use anyhow::Result;

use super::header::Header;
use super::resource::Data;
use super::source::{runtime, Source};
use crate::member::Member;

impl Source {
    /// Fetch set member.
    pub async fn set(&self, header: &Header) -> Result<()> {
        let remote = Self::replace_name(&self.patterns.grids, &header.name);

        self.load(remote.as_path(), Data::Set(header.name.to_owned()))
            .await?;

        Ok(())
    }

    /// Fetch member asynchronously.
    pub async fn member(&self, _header: &Header, _num: u32) -> Result<Member> {
        Ok(Member { blocks: vec![] })
    }
}

impl Member {
    /// Fetch member synchronously.
    pub fn fetch(source: &Source, header: &Header, num: u32) -> Result<Self> {
        runtime().block_on(source.member(header, num))
    }
}
