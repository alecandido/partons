//! Member of a set
use crate::{block::Block, data::lhapdf};

use serde::{Deserialize, Serialize};

use anyhow::Result;
use bincode::{Decode, Encode};
use bytes::Bytes;

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
}
