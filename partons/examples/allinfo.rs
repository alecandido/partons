use anyhow::Result;
use partons::configs::Configs;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Configs::load()?;

    let mut source = cfg.sources[0].clone();
    source.register_cache(cfg.data_path()?);
    let index = source.index().await?;

    let mut set = JoinSet::new();
    for header in index.into_iter() {
        let source = source.clone();
        set.spawn(async move { source.info(&header).await });
    }

    while let Some(res) = set.join_next().await {
        let desc = res??.set_desc;
        println!("\n\t{desc}");
    }

    eprintln!("Completed");

    Ok(())
}
