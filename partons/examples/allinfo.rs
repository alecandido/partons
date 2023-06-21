use anyhow::Result;
use partons::configs::Configs;

fn main() -> Result<()> {
    let cfg = Configs::load()?;

    let mut source = cfg.sources[0].clone();
    source.register_cache(cfg.data_path()?);
    let index = source.index()?;

    for header in index.into_iter() {
        let source = source.clone();
        let desc = source.info(&header)?;
        println!("\n\t{desc:#?}");
    }

    eprintln!("Completed");

    Ok(())
}
