// Load configs and download index file
use anyhow::Result;
use partons::configs::Configs;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Configs::load()?;

    // display the configs content
    println!("{:#?}", cfg);

    let mut source = cfg.sources[0].clone();
    source.register_cache(cfg.data_path()?);
    let index = source.index().await?;

    // display the first element, if non-empty
    if index.len() > 0 {
        println!("{:#?}", index[0]);
    }

    Ok(())
}
