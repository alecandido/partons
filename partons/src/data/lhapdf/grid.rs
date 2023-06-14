//! Parse legacy LHAPDF member files

use anyhow::Result;
use bytes::Bytes;

use crate::block::Block;
use crate::data::source::ConversionError;
use crate::member::Member;

pub(crate) struct Grid {
    blocks: Vec<Block>,
}

impl Grid {
    pub(crate) fn load(bytes: Bytes) -> Result<Self> {
        let blocks = vec![];
        Ok(Self { blocks })
    }
}

impl TryFrom<Grid> for Member {
    type Error = ConversionError;

    fn try_from(value: Grid) -> Result<Self, Self::Error> {
        Ok(Member {
            blocks: value.blocks,
        })
    }
}
