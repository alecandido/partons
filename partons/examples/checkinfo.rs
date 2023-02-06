use anyhow::Result;
use partons::configs::{self, /*data_path,*/ Configs};
//use tokio::task::JoinSet;

use std::fs;

//async fn fetch(header: SetHeader, source: Source, cache: PathBuf) -> (String, String) {
//    let info = header.fetch_info(&source, Some(cache.as_path())).await;
//    // println!("{:#?}", info);
//    let err = match info {
//        Ok(_) => "".to_owned(),
//        Err(e) => e.to_string(),
//    };
//    (header.name().to_owned(), err)
//}

#[tokio::main]
async fn main() -> Result<()> {
    let path = configs::Configs::path()?;
    let content = fs::read_to_string(path).unwrap();

    let cfg = toml::from_str::<Configs>(&content).unwrap();

    println!("{:#?}", cfg);

    //let source = &cfg.sources[0];
    //let cache = data_path()?;
    //let index = source.fetch_index(Some(cache.as_path())).await?;

    //let mut set = JoinSet::new();
    //for header in index.into_iter() {
    //    let source = source.clone();
    //    let cache = cache.clone();
    //    set.spawn(async move { fetch(header, source, cache).await });
    //}

    //while let Some(res) = set.join_next().await {
    //    // println!("{:#?}", res.unwrap());
    //}

    Ok(())
}
