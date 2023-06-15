//! Member of a set

use anyhow::{bail, Result};
use bincode::{Decode, Encode};
use bytes::Bytes;
use ndarray::Array1;
use serde::{Deserialize, Serialize};

use crate::block::Block;

/// Member of a set
///
/// This contains the whole member data, including the interpolation
/// [`Block`](crate::block::Block)s and further optional metadata.
#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
    pub(crate) blocks: Vec<Block>,
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
        q2: &Array1<f64>,
        nf: &Array1<u8>,
    ) -> Result<Array1<f64>> {
        if x.shape() != q2.shape() || x.shape() != nf.shape() {
            bail!("Incompatible array shapes.")
        }

        let mut values: Array1<f64> = Array1::zeros(x.raw_dim());
        values[0] = self.blocks[(nf[0] - 3) as usize].interp(pid[0], x[0], q2[0])?;

        Ok(values)
    }
}
