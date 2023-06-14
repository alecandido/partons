use anyhow::Result;
use bytes::Bytes;

use super::cache::Resource;
use crate::info::Info;
use crate::member::MemberWrapper;

pub(crate) mod grid;
pub(crate) mod info;

pub(crate) fn convert(bytes: Bytes, resource: &Resource) -> Result<Bytes> {
    match resource {
        Resource::Index => Ok(bytes),
        Resource::Info(_) => {
            let converted: Info = info::Info::load(bytes)?.try_into()?;
            let raw_bytes = serde_yaml::to_string(&converted)?.into_bytes();
            Ok(Bytes::copy_from_slice(&raw_bytes))
        }
        Resource::Set(_) => {
            todo!()
        }
        Resource::Grid(_, _) => {
            let converted = grid::Grid::load(bytes)?.try_into()?;
            let wrapper = MemberWrapper { member: converted };
            let raw_bytes = bincode::encode_to_vec(wrapper, bincode::config::standard())?;
            Ok(Bytes::copy_from_slice(&raw_bytes))
        }
    }
}
