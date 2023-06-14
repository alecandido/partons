//! Partons set

use std::{fs, path::PathBuf};

use anyhow::Result;

use crate::{
    data::{cache::Resource, source::Source},
    info::Info,
    member::Member,
};

struct Set {
    source: Source,
    path: PathBuf,
}

impl Set {
    fn name(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_owned()
    }

    fn info(&self) -> Result<Info> {
        let relative = Resource::Info(self.name()).path();
        let mut path = self.path.clone();
        path.push(&relative);

        Info::load(fs::read(path)?.into())
    }

    fn member(&self, num: u32) -> Result<Member> {
        let relative = Resource::Grid(self.name(), num).path();
        let mut path = self.path.clone();
        path.push(&relative);

        Member::load(fs::read(path)?.into())
    }
}
