use anyhow::Result;
use bytes::Bytes;

use crate::info::Info;

use super::cache::Resource;

pub(crate) mod grid;
pub(crate) mod info;

pub(crate) fn convert(bytes: Bytes, resource: &Resource) -> Result<Bytes> {
    match resource {
        Resource::Index => Ok(bytes),
        Resource::Info(_) => Ok(Bytes::copy_from_slice(
            &serde_yaml::to_string::<Info>(&info::Info::load(bytes)?.try_into()?)?.into_bytes(),
        )),
        Resource::Grid(_, _) => Ok(bytes),
    }
}
