use std::path::PathBuf;

use anyhow::{anyhow, Result};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::resource::Data;

use super::lhapdf;

/// Error during data conversion
#[derive(Error, Debug)]
pub enum ConversionError {
    /// Missing field from original value
    #[error("Missing field {0}")]
    MissingField(String),
    /// Type mismatched
    #[error("Missing field {0}")]
    FieldType(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Format {
    Native,
    Lhapdf,
}

impl Default for Format {
    fn default() -> Self {
        Format::Native
    }
}

impl Format {
    pub(crate) fn convert(&self, content: Bytes, data: &Data) -> Result<Bytes> {
        match self {
            Self::Native => {
                // TODO: move the content from original to new
                Ok(content)
            }
            Self::Lhapdf => lhapdf::convert(content, data),
        }
    }

    pub(crate) fn convert_name(&self, path: PathBuf) -> Result<String> {
        match self {
            Self::Native => path
                .file_name()
                .map(|s| s.to_str())
                .flatten()
                .map(|s| s.to_owned())
                .ok_or(anyhow!("...")),
            Self::Lhapdf => lhapdf::convert_name(path),
        }
    }
}
