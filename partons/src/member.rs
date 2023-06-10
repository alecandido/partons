//! Member of a set
use crate::{block::Block, data::lhapdf};

use serde::{Deserialize, Serialize};

use anyhow::Result;
use bytes::Bytes;

/// Member of a set
///
/// This contains the whole member data, including the interpolation
/// [`Block`](crate::block::Block)s and further optional metadata.
#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
    blocks: Vec<Block>,
}

impl Member {
    pub(crate) fn load(bytes: Bytes) -> Result<Self> {
        let lha = lhapdf::grid::Grid::load(bytes)?;
        Ok(Self { blocks: lha.blocks })
    }
}
