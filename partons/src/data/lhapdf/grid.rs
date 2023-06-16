//! Parse legacy LHAPDF member files

use std::collections::HashMap;

use anyhow::{bail, Result};
use bytes::Bytes;
use ndarray::{Array1, Array3};

use crate::block::Block;
use crate::data::format::ConversionError;
use crate::member::Member;

pub(crate) struct Grid {
    metadata: HashMap<String, String>,
    blocks: Vec<Block>,
}

impl Grid {
    fn block(_section: &str) -> Block {
        Block::new(
            Array1::from(vec![]),
            Array1::from(vec![]),
            Array1::from(vec![]),
            Array3::zeros((0, 0, 0)),
        )
    }

    pub(crate) fn load(bytes: Bytes) -> Result<Self> {
        let content = String::from_utf8(bytes.into())?;
        let mut sections = content.trim().split_terminator("---");

        let metadata = if let Some(header) = sections.next() {
            serde_yaml::from_slice(header.as_bytes())?
        } else {
            bail!("No section found in grid file.")
        };

        let mut blocks = Vec::new();
        for section in sections {
            blocks.push(Self::block(section));
        }

        Ok(Self { metadata, blocks })
    }
}

impl TryFrom<Grid> for Member {
    type Error = ConversionError;

    fn try_from(value: Grid) -> Result<Self, Self::Error> {
        Ok(Member {
            metadata: value.metadata,
            blocks: value.blocks,
        })
    }
}
