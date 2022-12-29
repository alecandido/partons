use super::remote::Source;

use anyhow::{bail, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use std::env::current_dir;
use std::path::PathBuf;
use std::process::Command;
use std::str;

const NAME: &str = "partons.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct Configs {
    pub sources: Vec<Source>,
}

impl Configs {
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
}

pub fn data_path() -> Result<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "Partons") {
        return Ok(proj_dirs.data_dir().to_owned());
    }

    bail!("Data path not found.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_test() {
        let cfg = r#"
[[sources]]
name = "pdfrepo"
url = "https://example.com/pdfs/"
index = "https://example.com/pdfs/pdfsets.index"

[[sources]]
name = "otherpdfrepo"
url = "https://example.com/others/pdfs/"
url = "https://example.com/others/pdfs.csv"
        "#;

        let loaded: Configs =
            toml::from_str(cfg).expect("Problem loading example TOML dump of configs.");

        assert_eq!(loaded.sources[0].url, "https://example.com/pdfs/");
        assert_eq!(
            loaded
                .sources
                .iter()
                .map(|s| &s.name)
                .collect::<Vec<&String>>(),
            ["pdfrepo", "otherpdfrepo"]
        );
    }
}
