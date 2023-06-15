use std::path::PathBuf;

use anyhow::{anyhow, bail, Result};
use bytes::Bytes;
use lazy_static::lazy_static;
use regex::Regex;

use super::cache::file::{INFO_NAME, MEMBER_PATTERN, MEMBER_PLACEHOLDER};
use super::resource::Data;
use crate::info::Info;
use crate::member::MemberWrapper;

pub(crate) mod grid;
pub(crate) mod info;

pub(crate) fn convert(bytes: Bytes, data: &Data) -> Result<Bytes> {
    match data {
        Data::Info(_) => {
            let converted: Info = info::Info::load(bytes)?.try_into()?;
            let raw_bytes = serde_yaml::to_string(&converted)?.into_bytes();
            Ok(Bytes::copy_from_slice(&raw_bytes))
        }
        Data::Member(_, _) => {
            let converted = grid::Grid::load(bytes)?.try_into()?;
            let wrapper = MemberWrapper { member: converted };
            let raw_bytes = bincode::encode_to_vec(wrapper, bincode::config::standard())?;
            Ok(Bytes::copy_from_slice(&raw_bytes))
        }
        _ => Ok(bytes),
    }
}

lazy_static! {
    static ref MEMBER_REGEX: Regex = Regex::new(r".*_(\d{4}).dat").unwrap();
}

pub(crate) fn convert_name(path: PathBuf) -> Result<String> {
    let file_name = path
        .file_name()
        .map(|s| s.to_str())
        .flatten()
        .map(|s| s.to_owned())
        .ok_or(anyhow!("File name missing."))?;

    if file_name.ends_with(".info") {
        Ok(INFO_NAME.to_owned())
    } else if let Some(captures) = MEMBER_REGEX.captures(&file_name) {
        let member = captures.get(1).unwrap().as_str();
        Ok(MEMBER_PATTERN.replace(MEMBER_PLACEHOLDER, &format!("{member:0>6}")))
    } else {
        bail!("...")
    }
}
