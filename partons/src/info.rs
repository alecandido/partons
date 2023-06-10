//! Store metadata of a set

use crate::data::lhapdf;

use anyhow::Result;
use bytes::Bytes;

/// Set metadata
pub struct Info {
    lhapdf: lhapdf::info::Info,
}

impl Info {
    pub(crate) fn load(bytes: Bytes) -> Result<Self> {
        Ok(Self {
            lhapdf: serde_yaml::from_slice(&bytes)?,
        })
    }

    pub fn description(&self) -> String {
        self.lhapdf.set_desc.clone()
    }
}
