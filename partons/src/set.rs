//! Partons set

use std::{fs, path::PathBuf};

use anyhow::Result;

use crate::{
    data::{cache::Resource, source::Source},
    info::Info,
    member::Member,
};

pub struct Set {
    source: Source,
    path: PathBuf,
}

impl Set {
    pub fn name(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_owned()
    }

    pub fn source(&self) -> String {
        self.source.name.clone()
    }

    pub fn info(&self) -> Result<Info> {
        let relative = Resource::Info(self.name()).path();
        let mut path = self.path.clone();
        path.push(&relative);

        Info::load(fs::read(path)?.into())
    }

    pub fn member(&self, num: u32) -> Result<Member> {
        let relative = Resource::Member(self.name(), num).path();
        let mut path = self.path.clone();
        path.push(&relative);

        Member::load(fs::read(path)?.into())
    }
}
