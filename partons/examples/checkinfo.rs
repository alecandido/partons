use anyhow::Result;
use partons::{
    configs::{self, Configs},
    remote::Source,
    set::SetHeader,
};
use tokio::task::JoinSet;

use std::fs;

async fn fetch(header: SetHeader, source: Source) -> (String, String) {
    let info = header.fetch_info(&source).await;
    // println!("{:#?}", info);
    let err = match info {
        Ok(_) => "".to_owned(),
        Err(e) => e.to_string(),
    };
    (header.name().to_owned(), err)
}

#[tokio::main]
async fn main() -> Result<()> {
    let path = configs::Configs::path()?;
    let content = fs::read_to_string(path).unwrap();

    let cfg = toml::from_str::<Configs>(&content).unwrap();

    println!("{:#?}", cfg);

    let source = &cfg.sources[0];
    let index = source.fetch_index().await?;

    let mut set = JoinSet::new();
    for header in index.into_iter() {
        let source = source.clone();
        set.spawn(async move { fetch(header, source).await });
    }

    while let Some(res) = set.join_next().await {
        println!("{:#?}", res.unwrap());
    }

    Ok(())
}
