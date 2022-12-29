use anyhow::Result;
use partons::configs::{self, data_path, Configs};

use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    let path = configs::Configs::path()?;
    let content = fs::read_to_string(path).unwrap();

    let cfg = toml::from_str::<Configs>(&content).unwrap();

    println!("{:#?}", cfg);

    let source = &cfg.sources[0];
    let cache = data_path();
    let index = source
        .fetch_index(cache.ok().as_ref().map(|p| p.as_path()))
        .await?;

    println!("{:#?}", index[0]);

    Ok(())
}
