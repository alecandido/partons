//! Manage data cache for a given source
//!
//! Each source has its own cache.
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::bail;
use anyhow::{anyhow, Context, Result};
use bytes::Bytes;
use flate2::read::GzDecoder;
use tar::Archive;

const INDEX_NAME: &str = "index.csv";
const INFO_NAME: &str = "info.yaml";
const SET_NAME: &str = "set.tar.gz";
const MEMBER_PLACEHOLDER: &str = "{member}";
const GRID_PATTERN: &str = "{member}.member.lz4";

#[derive(Debug)]
pub(crate) enum Resource {
    Index,
    Info(String),
    Set(String),
    Grid(String, u32),
}

impl Resource {
    fn dir_file(&self) -> (PathBuf, String) {
        let mut path_ = PathBuf::new();

        let file_name = match self {
            Self::Index => INDEX_NAME.to_owned(),
            Self::Info(name) => {
                path_.push(&name);
                INFO_NAME.to_owned()
            }
            Self::Set(name) => {
                path_.push(&name);
                SET_NAME.to_owned()
            }
            Self::Grid(name, member) => {
                path_.push(name);
                GRID_PATTERN.replace(MEMBER_PLACEHOLDER, &format!("{member:0>6}"))
            }
        };

        (path_, file_name)
    }

    pub(crate) fn path(&self) -> PathBuf {
        let (mut path_, file_name) = self.dir_file();

        path_.push(file_name);

        path_
    }

    pub(crate) fn raw_path(&self) -> PathBuf {
        let (mut path_, file_name) = self.dir_file();

        let prefix = format!("{}.", &Status::Raw.marker());
        let file_name = prefix + &file_name;

        path_.push(file_name);

        path_
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Resource::Index => write!(f, "Index"),
            Resource::Info(set) => write!(f, "Info: {set}"),
            Resource::Set(set) => write!(f, "Set: {set}"),
            Resource::Grid(set, num) => write!(f, "Grid: {set}-{num}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Status {
    Normal,
    Raw,
}

impl Status {
    pub(crate) fn marker(&self) -> String {
        match self {
            Self::Normal => "",
            Self::Raw => "raw",
        }
        .to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct Cache {
    path: PathBuf,
}

impl Cache {
    pub(crate) fn new(name: &str, data_path: PathBuf) -> Self {
        let mut path = data_path;
        path.push(name);
        Self { path }
    }

    fn absolute(&self, resource: &Resource) -> PathBuf {
        let mut abs = self.path.clone();
        abs.push(&resource.path());
        abs
    }

    pub(crate) fn exists(&self, resource: &Resource) -> bool {
        self.absolute(resource).exists()
    }

    pub(crate) fn write(&self, resource: &Resource, content: &Bytes) -> Result<PathBuf> {
        let location = self.absolute(resource);

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

    pub(crate) fn read(&self, resource: &Resource) -> Result<Bytes> {
        let location = self.absolute(resource);

        let content = match resource {
            Resource::Set(_) => location.to_str().unwrap().to_owned().into(),
            _ => fs::read(&location)?.into(),
        };
        println!("'{location:?}' loaded from cache");
        Ok(content)
    }

    pub(crate) fn unpack(&self, resource: &Resource) -> Result<PathBuf> {
        let mut location = self.absolute(resource);

        match resource {
            Resource::Set(_) => {
                let content = fs::read(&location)?;
                let tar = GzDecoder::new(&content[..]);
                let mut archive = Archive::new(tar);

                if !location.pop() {
                    bail!("Parent not available");
                };
                archive.unpack(&location).unwrap();
                Ok(location)
            }
            _ => Ok(location),
        }
    }

    pub fn sets(&self) -> Result<Vec<String>> {
        let mut sets_ = Vec::new();
        for entry in fs::read_dir(&self.path)? {
            let os_name = entry?.file_name();
            let name = os_name.to_str().context("Invalid set name encountered.")?;
            sets_.push(format!("{name}"))
        }
        Ok(sets_)
    }
}
