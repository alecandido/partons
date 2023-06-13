use anyhow::Result;
use partons::configs::Configs;
use partons::info::Info;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let mut path = env::current_dir()?;
    path.push("../partons.toml");
    let configs = Configs::new(path)?;
    let index = configs.sources[0].index().await?;
    let entry = index.get("NNPDF40_nnlo_as_01180")?;
    let info: Info = configs.sources[0].info(&entry).await?;
    println!("{info:#?}");
    Ok(())
}
