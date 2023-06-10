//! Manage data cache for a given source
//!
//! Each source has its own cache.

use anyhow::Result;

use std::path::PathBuf;

const INDEX_NAME: &str = "index.csv";
const INFO_NAME: &str = "info.yaml";
const MEMBER_PLACEHOLDER: &str = "{member}";
const GRID_PATTERN: &str = "{member}.member.lz4";

pub(crate) enum Resource {
    Index,
    Info(String),
    Grid(String, u32),
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

    pub(crate) fn locate(&self, resource: &Resource) -> Result<PathBuf> {
        let mut buf = self.path.clone();
        buf.push(Self::local(&resource));

        Ok(buf)
    }

    fn local(resource: &Resource) -> PathBuf {
        let mut path = PathBuf::new();

        match resource {
            Resource::Index => path.push(INDEX_NAME),
            Resource::Info(name) => {
                path.push(&name);
                path.push(INFO_NAME)
            }
            Resource::Grid(name, member) => {
                path.push(name);
                path.push(GRID_PATTERN.replace(MEMBER_PLACEHOLDER, &format!("{member:0>6}")))
            }
        };

        path
    }
}
