// Load configs and download index file
use anyhow::Result;
use partons::configs::Configs;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Configs::load()?;

    // display the configs content
    println!("{:#?}", cfg);

    let source = &cfg.sources()[0];
    let index = source
        .index(cfg.data_path().as_ref().ok().map(|p| p.as_path()))
        .await?;

    // display the first element, if non-empty
    if index.len() > 0 {
        println!("{:#?}", index[0]);
    }

    Ok(())
}
