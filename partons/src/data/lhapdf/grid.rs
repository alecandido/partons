//! Parse legacy LHAPDF member files

use anyhow::{bail, Result};
use bytes::Bytes;
use ndarray::{Array1, Array3};

use crate::block::Block;
use crate::data::format::ConversionError;
use crate::member::{Member, Metadata};

pub(crate) struct Grid {
    metadata: Metadata,
    blocks: Vec<Block>,
}

impl Grid {
    fn metadata(section: Option<&str>) -> Result<Metadata> {
        if let Some(header) = section {
            Ok(serde_yaml::from_slice(header.as_bytes())?)
        } else {
            bail!("No section found in grid file.")
        }
    }

    fn sequence<T>(line: Option<&str>) -> Result<Array1<T>> {
        if let Some(_text) = line {
            todo!()
        } else {
            bail!("")
        };
    }

    fn table(lines: Option<&str>) -> Result<Array3<f64>> {
        if let Some(_text) = lines {
            todo!()
        } else {
            bail!("")
        };
    }

    fn block(section: &str) -> Result<Block> {
        let mut split = section.splitn(3, '\n');

        let xgrid = Self::sequence(split.next())?;
        let mu2grid = Self::sequence(split.next())?;
        let pids = Self::sequence(split.next())?;

        let values = Self::table(split.next())?;

        Ok(Block::new(pids, xgrid, mu2grid, values))
    }

    pub(crate) fn load(bytes: Bytes) -> Result<Self> {
        let content = String::from_utf8(bytes.into())?;
        let mut sections = content.trim().split_terminator("---");

        let metadata = Self::metadata(sections.next())?;

        let mut blocks = Vec::new();
        for section in sections {
            blocks.push(Self::block(section)?);
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
