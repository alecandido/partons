use std::collections::HashMap;
use std::fmt::Debug;

use anyhow::Result;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::data::source::ConversionError::{self, FieldType, MissingField};
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
    ($name:ident: String = $value:ident[$lha_name:literal]) => {
        extract!($name, String, $lha_name, $value);
        let $name = $name.map(|s| s.to_owned());
    };
    ($name:ident: Option<$type:ident> = $value:ident[$lha_name:literal]) => {
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
        convert!(description: Option<String> = value["SetDesc"]);
        convert!(authors: Option<String> = value["Authors"]);
        convert!(year: u64 = value["Year"]);
        Ok(info::Info {
            id,
            description,
            authors,
            year,
            more_members: value.0,
        })
    }
}

// This should be i32, but unfortunately it is not honored by all sets:
// https://lhapdfsets.web.cern.ch/current/JAM20-SIDIS_FF_hadron_nlo/JAM20-SIDIS_FF_hadron_nlo.info
pub type PID = String;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DetailedInfo {
    #[serde(default, rename = "Reference")]
    reference: Option<String>,
    #[serde(default, rename = "Format")]
    format: Option<String>, // TODO: replace with enum
    #[serde(rename = "DataVersion")]
    data_version: u32,
    #[serde(rename = "NumMembers")]
    num_members: u32,
    #[serde(default, rename = "Particle")]
    particle: Option<PID>,
    #[serde(rename = "Flavors")]
    flavors: Vec<PID>,
    #[serde(default, rename = "OrderQCD")]
    order_qcd: Option<u32>,
    #[serde(rename = "FlavorScheme")]
    flavor_scheme: String, // TODO: replace with enum
    #[serde(default, rename = "NumFlavors")]
    num_flavors: Option<u32>,
    #[serde(default, rename = "ErrorType")]
    error_type: Option<String>, // TODO: replace with enum
    #[serde(rename = "XMin")]
    x_min: f64,
    #[serde(rename = "XMax")]
    x_max: f64,
    #[serde(rename = "QMin")]
    q_min: f64,
    #[serde(rename = "QMax")]
    q_max: f64,
    #[serde(default, rename = "MZ")]
    mz: Option<f64>,
    #[serde(default, rename = "MUp")]
    m_up: Option<f64>,
    #[serde(default, rename = "MDown")]
    m_down: Option<f64>,
    #[serde(default, rename = "MStrange")]
    m_strange: Option<f64>,
    #[serde(default, rename = "MCharm")]
    m_charm: Option<f64>,
    #[serde(default, rename = "MBottom")]
    m_bottom: Option<f64>,
    #[serde(default, rename = "MTop")]
    m_top: Option<f64>,
    #[serde(default, rename = "AlphaS_MZ")]
    alpha_s_mz: Option<f64>,
    #[serde(default, rename = "AlphaS_OrderQCD")]
    alpha_s_order_qcd: Option<u32>,
    #[serde(default, rename = "AlphaS_Type")]
    alpha_s_type: Option<String>, // TODO: replace with enum
    #[serde(default, rename = "AlphaS_Qs")]
    alpha_s_qs: Option<Vec<f64>>,
    #[serde(default, rename = "AlphaS_Vals")]
    alpha_s_vals: Option<Vec<f64>>,
    #[serde(default, rename = "AlphaS_Lambda4")]
    alpha_s_lambda4: Option<f64>,
    #[serde(default, rename = "AlphaS_Lambda5")]
    alpha_s_lambda5: Option<f64>,
    #[serde(default, rename = "Extrapolator")]
    extrapolator: Option<String>, // TODO: replace with enum
    #[serde(default)]
    _more: HashMap<String, String>,
}
