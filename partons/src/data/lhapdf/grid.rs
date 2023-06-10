//! Parse legacy LHAPDF member files

use anyhow::Result;
use bytes::Bytes;

use crate::block::Block;

pub(crate) struct Grid {
    pub(crate) blocks: Vec<Block>,
}

impl Grid {
    pub(crate) fn load(bytes: Bytes) -> Result<Self> {
        let blocks = vec![];
        Ok(Self { blocks })
    }
}
