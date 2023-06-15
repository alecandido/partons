//! Parse legacy LHAPDF member files

use anyhow::Result;
use bytes::Bytes;
use ndarray::{Array1, Array3};

use crate::block::Block;
use crate::data::source::ConversionError;
use crate::member::Member;

pub(crate) struct Grid {
    blocks: Vec<Block>,
}

impl Grid {
    pub(crate) fn load(_bytes: Bytes) -> Result<Self> {
        let blocks = vec![Block::new(
            Array1::from(vec![]),
            Array1::from(vec![]),
            Array1::from(vec![]),
            Array3::zeros((0, 0, 0)),
        )];
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
