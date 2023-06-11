//! Manage data cache for a given source
//!
//! Each source has its own cache.

use anyhow::{anyhow, Result};
use bytes::Bytes;

use std::fs;
use std::path::{Path, PathBuf};

const INDEX_NAME: &str = "index.csv";
const INFO_NAME: &str = "info.yaml";
const MEMBER_PLACEHOLDER: &str = "{member}";
const GRID_PATTERN: &str = "{member}.member.lz4";

pub(crate) enum Resource {
    Index,
    Info(String),
    Grid(String, u32),
}

impl Resource {
    pub(crate) fn path(&self) -> PathBuf {
        let mut path_ = PathBuf::new();

        match self {
            Resource::Index => path_.push(INDEX_NAME),
            Resource::Info(name) => {
                path_.push(&name);
                path_.push(INFO_NAME)
            }
            Resource::Grid(name, member) => {
                path_.push(name);
                path_.push(GRID_PATTERN.replace(MEMBER_PLACEHOLDER, &format!("{member:0>6}")))
            }
        };

        path_
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Status {
    Normal,
    Raw,
}

impl Status {
    pub(crate) fn suffix(&self) -> String {
        match self {
            Self::Normal => "",
            Self::Raw => ".raw",
        }
        .to_owned()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Cache {
    path: PathBuf,
}

impl Cache {
    pub(crate) fn new(name: &str, data_path: PathBuf) -> Self {
        let mut path = data_path;
        path.push(name);
        Self { path }
    }

    fn absolute(&self, relative: &Path) -> PathBuf {
        let mut abs = self.path.clone();
        abs.push(relative);
        abs
    }

    pub(crate) fn write(&self, relative: &Path, content: &Bytes) -> Result<PathBuf> {
        let location = self.absolute(relative);

        // TODO: move old to trash bin
        fs::create_dir_all(
            location
                .parent()
                .ok_or(anyhow!("Fail to access parent of '{location:?}'"))?,
        )?;

        fs::write(&location, &content)?;
        println!("'{location:?}' cached");

        Ok(location)
    }

    pub(crate) fn read(&self, relative: &Path) -> Result<Bytes> {
        let location = self.absolute(relative);

        let content = fs::read(&location)?.into();
        println!("'{location:?}' loaded from cache");
        Ok(content)
    }
}
