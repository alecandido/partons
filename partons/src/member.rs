//! Member of a set

use std::collections::HashMap;
use std::fmt::{self, Display};

use anyhow::{bail, Result};
use bincode::{Decode, Encode};
use bytes::Bytes;
use ndarray::Array1;
use serde::{Deserialize, Serialize};

use crate::block::Block2;

pub(crate) type Metadata = HashMap<String, String>;

/// Member of a set
///
/// This contains the whole member data, including the interpolation
/// [blocks](crate::block::Block2) and further optional metadata.
#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
    pub(crate) metadata: Metadata,
    pub(crate) blocks: Vec<Block2>,
}

#[derive(Decode, Encode)]
pub(crate) struct MemberWrapper {
    #[bincode(with_serde)]
    pub(crate) member: Member,
}

impl Member {
    pub(crate) fn load(bytes: Bytes) -> Result<Self> {
        let decoded: MemberWrapper =
            bincode::decode_from_slice(&bytes, bincode::config::standard())?.0;
        Ok(decoded.member)
    }

    /// Return interpolated values
    pub fn evaluate(
        &self,
        pid: &Array1<i32>,
        x: &Array1<f64>,
        mu2: &Array1<f64>,
        nf: &Array1<u8>,
    ) -> Result<Array1<f64>> {
        if x.shape() != mu2.shape() || x.shape() != nf.shape() {
            bail!("Incompatible array shapes.")
        }

        let mut values: Array1<f64> = Array1::zeros(x.raw_dim());
        values[0] = self.blocks[(nf[0] - 3) as usize].interp(x[0], mu2[0])?;

        Ok(values)
    }
}

impl Display for Member {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn beautify<T: Serialize>(map: &T) -> String {
            let metadata = serde_yaml::to_string(map).unwrap().replace('\n', "\n    ");
            let metadata = metadata.trim();
            metadata.to_owned()
        }

        let metadata = beautify(&self.metadata);
        let shapes: Vec<_> = self
            .blocks
            .iter()
            .map(|b| {
                let v: HashMap<&str, usize> = ["pids", "xs", "mu2s"]
                    .iter()
                    .zip(b.values.shape())
                    .map(|(s, l)| (s.to_owned(), l.clone()))
                    .collect();
                v
            })
            .collect();
        let shapes = beautify(&shapes);
        write!(
            f,
            "
Member(
    ---
    {metadata}
    ---
    {shapes}
)"
        )
    }
}
