//! Member of a set

use std::collections::HashMap;
use std::fmt::{self, Display};
use std::vec;

use anyhow::Result;
use bincode::{Decode, Encode};
use bytes::Bytes;
use itertools::izip;
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
    pub(crate) indices: Vec<Array1<i32>>,
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

    fn block_index(&self, pid: i32, nf: u8) -> usize {
        pid * 4 + (nf - 3)
    }

    /// Return interpolated values.
    ///
    /// The parameters for each value returned are those for the corresponding index in each of the
    /// input vectors.
    /// The length of the result is the minimum length of the parameters vectors.
    ///
    /// #### Note
    ///
    /// A corresponding function for scalar values is not provided, since it can be trivially
    /// obtained by wrapping them in vectors and unwrapping the result.
    /// The rational to provide the vector version, rather than the scalar, as primitive is to
    /// allow potential optimizations for the evaluation of multiple values.
    pub fn evaluate(
        &self,
        x: &Vec<f64>,
        mu2: &Vec<f64>,
        pid: &Vec<i32>,
        nf: &Vec<u8>,
    ) -> Result<Vec<f64>> {
        let mut values = vec![];

        for (y, m, p, n) in izip!(x, mu2, pid, nf) {
            values.push(self.blocks[self.block_index(*p, *n)].interp(*y, *m)?);
        }

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
