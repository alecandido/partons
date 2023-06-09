//! Manage partons configurations.
//!
//! Example for LHAPDF source:
//! ```text
#![doc = include_str!("../../partons.toml")]
//! ```
use super::data::source::Source;

use anyhow::{bail, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use std::env::current_dir;
use std::path::PathBuf;
use std::process::Command;
use std::{fs, str};

/// Name of the configuration file
pub const NAME: &str = "partons.toml";

/// Application configurations
///
/// User configurations are directly deserialized from files in this structure.
#[derive(Serialize, Deserialize, Debug)]
pub struct Configs {
    sources: Vec<Source>,
}

impl Configs {
    /// Loads configs from `path`.
    ///
    /// ```
    /// use partons::configs::Configs;
    /// use anyhow::Result;
    /// use std::env;
    ///
    /// fn main() -> Result<()> {
    ///     let mut path = env::current_dir()?;
    ///     path.push("../partons.toml");
    ///     let configs = Configs::new(path)?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// To load an automatically detected configuration file use [`Configs::load`].
    pub fn new(path: PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let cfg = toml::from_str::<Self>(&content)?;
        Ok(cfg)
    }

    /// Determine configs path
    ///
    /// The following locations are probed to check for a file named [`NAME`]:
    /// - current directory
    /// - git root folder (if in a Git folder)
    /// - the "Partons" configuration directory
    ///     - this is system-dependent (on Linux it would be `$XDG_CONFIG_HOME/partons`)
    ///     - check [`directories`] crate for further details
    ///
    /// As soon as such a file is detected, its path is returned, without probing any further
    /// location.
    pub fn path() -> Result<PathBuf> {
        let mut paths = Vec::new();

        // Add cwd
        paths.push(current_dir()?);

        // Add git root folder, if any
        if let Ok(output) = Command::new("git")
            .args(&["rev-parse", "--show-toplevel"])
            .output()
        {
            if output.status.success() {
                paths.push(PathBuf::from(str::from_utf8(&output.stdout)?.trim()))
            }
        }

        // Add user config dir
        if let Some(proj_dirs) = ProjectDirs::from("", "", "Partons") {
            paths.push(proj_dirs.config_dir().to_path_buf());
        }

        // look for existing configs
        for mut p in paths.into_iter() {
            p.push(NAME);
            if p.exists() {
                return Ok(p);
            }
        }

        bail!("No configuration file found.")
    }

    /// Load configs from autodected path.
    pub fn load() -> Result<Self> {
        Self::new(Self::path()?)
    }

    /// Determine data location.
    ///
    /// # Note
    /// At present time this is actually not configurable.
    pub fn data_path(&self) -> Result<PathBuf> {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "Partons") {
            return Ok(proj_dirs.data_dir().to_owned());
        }

        bail!("Data path not found.")
    }

    /// Return configured sources.
    pub fn sources(&self) -> &Vec<Source> {
        &self.sources
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialization() {
        // Just test that is able to deserialize the structure

        let cfg = r#"
        [[sources]]
        name = "pdfrepo"
        url = "https://example.com/pdfs/"
        index = "https://example.com/pdfs/pdfsets.index"
        
        [[sources]]
        name = "otherpdfrepo"
        url = "https://example.com/others/pdfs/"
        index = "https://example.com/others/pdfs.csv"
        "#;

        let loaded: Configs =
            toml::from_str(cfg).expect("Problem loading example TOML dump of configs.");

        assert_eq!(loaded.sources().len(), 2);
    }
}
