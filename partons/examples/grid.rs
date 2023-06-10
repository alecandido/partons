// Load configs and download index file
use anyhow::Result;
use partons::configs::Configs;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Configs::load()?;

    let mut source = cfg.sources[0].clone();
    source.register_cache(cfg.data_path()?);
    let index = source.index().await?;

    // display the first element, if non-empty
    for set in ["NNPDF40_nnlo_as_01180", "MSHT20nnlo_as118", "CT18NNLO"] {
        println!("");
        let header = index.get(set)?;
        source.grid(&header, 0).await?;
    }

    Ok(())
}
