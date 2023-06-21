// Load configs and download index file
use anyhow::Result;
use partons::configs::Configs;

async fn main() -> Result<()> {
    let cfg = Configs::load()?;

    let mut source = cfg.sources[0].clone();
    source.register_cache(cfg.data_path()?);
    let index = source.index().await?;

    // display the first element, if non-empty
    for set in ["NNPDF40_nnlo_as_01180"] {
        //, "MSHT20nnlo_as118", "CT18NNLO"] {
        let header = index.get(set)?;
        let mut set = source.set(&header).await?;
        println!("{set:#?}");
        let grid0 = set.member(0).await?;
        println!("{grid0}");
    }

    Ok(())
}
