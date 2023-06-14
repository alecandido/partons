//! Store metadata of a set

use std::{collections::HashMap, hash::Hash};

use anyhow::Result;
use bytes::Bytes;
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use serde_yaml::Value;

/// Set metadata
#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    /// Numerical identifier, within the source
    pub id: Option<u64>,
    /// Unstructured description
    pub description: String,
    /// List of authors
    pub authors: String,
    /// Fitting year
    pub year: Option<u64>,
    /// Extra information to keep
    ///
    /// This field is here to store, mainly, legacy fields, that should be kept, but
    /// TODO: find a better name, for the time being I'm using PineAPPL's one
    pub more_members: HashMap<String, Value>,
}

impl Info {
    pub(crate) fn load(bytes: Bytes) -> Result<Self> {
        Ok(serde_yaml::from_slice(&bytes)?)
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
