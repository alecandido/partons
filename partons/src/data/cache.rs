//! Manage data cache for a given source
//!
//! Each source has its own cache.

use anyhow::Result;

use std::path::{Path, PathBuf};

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

    pub(crate) fn locate(&self, path: &Path) -> Result<PathBuf> {
        let mut buf = self.path.clone();
        buf.push(path);

        Ok(buf)
    }
}
