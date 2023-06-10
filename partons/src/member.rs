//! Member of a set
use serde::{Deserialize, Serialize};

use anyhow::Result;
use bytes::Bytes;

#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
    lhapdf: Option<i32>,
}

impl Member {
    pub(crate) fn load(bytes: Bytes) -> Result<Self> {
        Ok(Self {
            lhapdf: serde_yaml::from_slice(&bytes)?,
        })
    }
}
