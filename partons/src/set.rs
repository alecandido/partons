//! Partons set

use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::{
    data::{header::Header, source::Source},
    info::Info,
    member::Member,
};

/// Partons set.
pub struct Set {
    source: Source,
    header: Header,
    info: Option<Info>,
    members: HashMap<u32, Member>,
}

impl Set {
    /// Set name.
    ///
    /// Usually unique enough to identify the set within a source.
    pub fn name(&self) -> String {
        self.header.name.to_owned()
    }

    /// Original source from which the data have been fetched.
    pub fn source(&self) -> String {
        self.source.name.clone()
    }

    /// Metadata.
    pub fn info(&mut self) -> Result<&Info> {
        if let None = self.info {
            self.info = Some(Info::fetch(&mut self.source, &self.header)?);
        };

        self.info.as_ref().ok_or(anyhow!("..."))
    }

    /// Retrieve a set member.
    pub fn member(&mut self, num: u32) -> Result<&Member> {
        if let None = self.members.get(&num) {
            let member = Member::fetch(&mut self.source, &self.header, num)?;
            self.members.insert(num, member);
        }

        // TODO: I have to call twice get in any case, because if I hold the reference to the first
        // returned value, I can no longer mutate self.members, and even if I return it, the borrow
        // checker believes I still have the borrow (this shouldn't be lexical lifetimes in action,
        // since I'm developing on Rust 1.70.0, but then I don't know why...)
        Ok(&self.members.get(&num).unwrap())
    }
}
