use std::{fs, time::Duration};

use anyhow::Result;
use partons::configs::{self, Configs};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<()> {
    let path = configs::Configs::path()?;
    let content = fs::read_to_string(path).unwrap();

    let cfg = toml::from_str::<Configs>(&content).unwrap();

    println!("{:#?}", cfg);

    let source = &cfg.sources[0];
    let index = source.fetch_index().await?;

    let mut set = JoinSet::new();
    for (i, header) in index.into_iter().enumerate() {
        println!("{:#?}", header);
        let source = source.clone();
        set.spawn(async move {
            // slow down requests to pass DDoS filters
            let nanos = i as f64 * 1e8;
            tokio::time::sleep(Duration::new(
                (nanos / 1e9).trunc() as u64,
                (nanos.fract() * 1e9) as u32,
            ))
            .await;
            let err = match header.fetch_info(&source).await {
                Ok(_) => "".to_owned(),
                Err(e) => e.to_string(),
            };
            // println!("{:#?}", info);
            (header.name().to_owned(), err)
        });
    }

    while let Some(res) = set.join_next().await {
        println!("{:#?}", res.unwrap());
    }

    Ok(())
}
