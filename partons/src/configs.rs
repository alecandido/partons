use anyhow::{bail, Result};
use directories::ProjectDirs;

use std::env::current_dir;
use std::path::PathBuf;
use std::process::Command;
use std::str;

const NAME: &str = "partons.toml";

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

pub fn data_path() -> PathBuf {
    return PathBuf::from("/tmp/foo/bar.txt");
}
