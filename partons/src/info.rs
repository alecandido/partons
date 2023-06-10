//! Store metadata of a set

use crate::data::lhapdf;

use anyhow::Result;
use bytes::Bytes;
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};

/// Set metadata
#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    lhapdf: lhapdf::info::Info,
}

impl Info {
    pub(crate) fn load(bytes: Bytes) -> Result<Self> {
        Ok(Self {
            lhapdf: serde_yaml::from_slice(&bytes)?,
        })
    }

    pub fn description(&self) -> String {
        self.lhapdf.set_desc.clone()
    }
}

/// A set author
pub struct Author {
    name: String,
    address: String,
}

impl Serialize for Author {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let name = &self.name;
        let address = &self.address;
        serializer.serialize_str(&format!("{name} <{address}>"))
    }
}

impl<'de> Deserialize<'de> for Author {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Name,
            Address,
        }

        struct AuthorVisitor;

        impl<'de> Visitor<'de> for AuthorVisitor {
            type Value = Author;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("author representation 'Whatever Name <email@address.net>'")
            }

            fn visit_str<E>(self, string: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let elems: Vec<&str> = string.split(&['<', '>']).collect();
                let mut it = elems.iter();
                let mut trim_next = |pos| {
                    Ok(it
                        .next()
                        .ok_or_else(|| de::Error::invalid_length(pos, &self))?
                        .trim()
                        .to_owned())
                };
                let name = trim_next(0)?;
                let address = trim_next(1)?;
                Ok(Author { name, address })
            }
        }

        const FIELDS: &'static [&'static str] = &["name", "address"];
        deserializer.deserialize_struct("Author", FIELDS, AuthorVisitor)
    }
}
