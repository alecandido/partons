use std::collections::HashMap;

use anyhow::Result;
use bytes::Bytes;
use serde_yaml::Value;

use crate::data::format::ConversionError::{self, FieldType, MissingField};
use crate::info;

pub struct Info(HashMap<String, Value>);

impl Info {
    pub(crate) fn load(bytes: Bytes) -> Result<Self> {
        Ok(Self(serde_yaml::from_slice(&bytes)?))
    }
}

macro_rules! extract {
    ($name:ident, $type:ident, $lha_name:literal, $value:ident) => {
        let wrong_type = Err(FieldType(stringify!($name).to_owned()));
        let $name = $value
            .0
            .get($lha_name)
            .map(|v| {
                let Value::$type(num) = v else { return wrong_type};
                Ok(num)
            })
            .transpose()?;
    };
}

macro_rules! convert {
    ($name:ident: u64 = $value:ident[$lha_name:literal]) => {
        extract!($name, Number, $lha_name, $value);
        let $name = $name.map(|n| n.as_u64().unwrap());
    };
    ($name:ident: i64 = $value:ident[$lha_name:literal]) => {
        extract!($name, Number, $lha_name, $value);
        let $name = $name.map(|n| n.as_i64().unwrap());
    };
    ($name:ident: PID = $value:ident[$lha_name:literal]) => {
        convert!($name: i64 = $value[$lha_name]);
    };
    ($name:ident: f64 = $value:ident[$lha_name:literal]) => {
        extract!($name, Number, $lha_name, $value);
        let $name = $name.map(|n| n.as_f64().unwrap());
    };
    ($name:ident: String = $value:ident[$lha_name:literal]) => {
        extract!($name, String, $lha_name, $value);
        let $name = $name.map(|s| s.to_owned());
    };
    (Some($name:ident: $type:ident) = $value:ident[$lha_name:literal]) => {
        convert!($name: $type = $value[$lha_name]);
        let missing = MissingField(stringify!($name).to_owned());
        let $name = $name.ok_or(missing)?;
    };
}

// let missing = ;
impl TryFrom<Info> for info::Info {
    type Error = ConversionError;

    fn try_from(value: Info) -> Result<Self, Self::Error> {
        convert!(id: u64 = value["SetIndex"]);
        convert!(Some(description: String) = value["SetDesc"]);
        convert!(Some(authors: String) = value["Authors"]);
        convert!(year: u64 = value["Year"]);
        convert!(reference: String = value["Reference"]);
        convert!(particle: PID = value["Particle"]);
        convert!(Some(order_qcd: u64) = value["OrderQCD"]);
        convert!(error_type: String = value["ErrorType"]);
        convert!(data_version: i64 = value["Note"]);
        convert!(note: String = value["Note"]);
        Ok(info::Info {
            id,
            description,
            authors,
            year,
            reference,
            particle,
            order: (order_qcd, 0),
            error_type,
            data_version,
            note,
            more_members: value.0,
        })
    }
}
