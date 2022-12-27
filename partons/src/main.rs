use std::fs;

use anyhow::Result;
use partons::configs::{self, Configs};

fn main() -> Result<()> {
    let path = configs::Configs::path()?;
    let content = fs::read_to_string(path).unwrap();

    let cfg = toml::from_str::<Configs>(&content).unwrap();

    println!("{:#?}", cfg);

    Ok(())
}
