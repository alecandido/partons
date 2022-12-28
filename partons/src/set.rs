use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{info::Info, remote::Source};

#[derive(Serialize, Deserialize, Debug)]
pub struct SetHeader {
    id: u32,
    name: String,
    number: u32,
}

const NAME_PLACEHOLDER: &str = "{name}";

impl SetHeader {
    pub fn new(id: u32, name: String, number: u32) -> Self {
        Self { id, name, number }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub async fn fetch_info(&self, source: &Source) -> Result<Info> {
        let pattern = source.patterns.info.clone();
        source
            .fetch_info(&pattern.replace(NAME_PLACEHOLDER, &self.name))
            .await
    }
}
