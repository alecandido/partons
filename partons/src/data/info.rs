use anyhow::{anyhow, Result};

use super::header::Header;
use super::resource::Data;
use super::source::{runtime, Source};
use crate::info::Info;

impl Source {
    /// Fetch set metadata.
    ///
    /// ```
    /// # use partons::configs::Configs;
    /// # use partons::info::Info;
    /// # use anyhow::Result;
    /// # use std::env;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// #     let mut path = env::current_dir()?;
    /// #     path.push("../partons.toml");
    ///       let configs = Configs::new(path)?;
    ///       let mut source = configs.sources[0].clone();
    ///       source.register_cache(configs.data_path()?);
    ///       let index = source.index().await?;
    ///       let entry = index.get("NNPDF40_nnlo_as_01180")?;
    ///       let info: Info = source.info(&entry).await?;
    /// #     Ok(())
    /// # }
    /// ```
    pub async fn info(&self, header: &Header) -> Result<Info> {
        let remote = Self::replace_name(&self.patterns.info, &header.name);
        let content = self
            .load(remote.as_path(), Data::Info(header.name.to_owned()))
            .await?;

        Info::load(content).map_err(|err| {
            anyhow!(
                "Failed to parse info file for {}:\n\t{:?}",
                header.identifier(),
                err
            )
        })
    }
}

impl Info {
    /// Fetch info synchronously.
    pub fn fetch(source: &Source, header: &Header) -> Result<Self> {
        runtime().block_on(source.info(header))
    }
}
