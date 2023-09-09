//! Parse legacy LHAPDF member files

use std::fmt::Debug;
use std::str::FromStr;

use anyhow::{bail, Result};
use bytes::Bytes;
use ndarray::{Array1, Array2};

use crate::block::Block2;
use crate::data::format::ConversionError;
use crate::member::{Member, Metadata};

pub(crate) struct Grid {
    metadata: Metadata,
    blocks: Vec<Block2>,
}

impl Grid {
    fn metadata(section: Option<&str>) -> Result<Metadata> {
        if let Some(header) = section {
            Ok(serde_yaml::from_slice(header.as_bytes())?)
        } else {
            bail!("No section found in grid file.")
        }
    }

    /// Parse 1 dimensional sequence of values from a space separated string
    fn sequence<T: FromStr>(line: Option<&str>) -> Result<Array1<T>>
    where
        <T as FromStr>::Err: Debug,
    {
        if let Some(text) = line {
            let nums = text
                .trim()
                .split(" ")
                .into_iter()
                .map(|v| str::parse(v).unwrap())
                .collect();
            Ok(Array1::from_vec(nums))
        } else {
            bail!("");
        }
    }

    /// Parse 2 dimensional table of values from a space and line separated string
    fn table(lines: Option<&str>) -> Result<Array2<f64>> {
        if let Some(text) = lines {
            let mut rows = Vec::new();
            for line in text.lines() {
                let nums: Vec<f64> = line
                    .trim()
                    .split(" ")
                    .filter(|v| v.len() > 0)
                    .into_iter()
                    .map(|v| str::parse(v).unwrap())
                    .collect();
                rows.push(nums);
            }
            let shape = (rows.len(), rows[0].len());
            let values = rows.concat();
            Ok(Array2::from_shape_vec(shape, values)?)
        } else {
            bail!("")
        }
    }

    /// Split the table of values in a sequence of 2 dimensional array, and lift the dimension
    /// related to the discrete quantity (PID) as the outermost.
    ///
    /// This is required in partons, because the innermost indices are dedicated to the
    /// interpolated dimensions.
    fn values(values: Array2<f64>, xs: usize) -> Result<Vec<Array2<f64>>> {
        let &[points, pids] = values.shape() else {
            bail!("")
        };
        let mu2s = points / xs;
        let lha_shaped = values.into_shape((xs, mu2s, pids))?;
        // TODO: solve the following horrible memory duplication, and horrible loop
        let mut native_shaped = vec![Array2::zeros((xs, mu2s)); pids];
        for ((x, mu, p), val) in lha_shaped.indexed_iter() {
            native_shaped[p][(x, mu)] = *val;
        }
        Ok(native_shaped)
    }

    fn block(section: &str) -> Result<(Vec<Block2>, Array1<i32>)> {
        let mut split = section.trim().splitn(4, '\n');

        let xgrid = Self::sequence(split.next())?;
        let mu2grid = Self::sequence(split.next())?;
        let pids = Self::sequence(split.next())?;

        let values = Self::values(Self::table(split.next())?, xgrid.len())?;

        Ok((
            values
                .into_iter()
                .map(|ar| Block2::new((xgrid.clone(), mu2grid.clone()), ar))
                .collect(),
            pids,
        ))
    }

    pub(crate) fn load(bytes: Bytes) -> Result<Self> {
        let content = String::from_utf8(bytes.into())?;
        let mut sections = content.trim().split_terminator("---");

        let metadata = Self::metadata(sections.next())?;

        let mut blocks = Vec::new();
        let mut pids = Vec::new();
        for section in sections {
            let (block, pids_) = Self::block(section)?;
            blocks.extend(block);
            pids.extend(pids_);
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
