//! Set files metadata
//!
//! This should not be confused with the Info, giving furher information about the set, its
//! content, and the related physics. This headers are only minimal descriptions required for
//! transferring data.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) number: u32,
}

pub(crate) const NAME_PLACEHOLDER: &str = "{name}";

impl Header {
    pub(crate) fn new(id: u32, name: String, number: u32) -> Self {
        Self { id, name, number }
    }

    pub(crate) fn identifier(&self) -> String {
        format!("{}:{}", self.name, self.id)
    }
}
