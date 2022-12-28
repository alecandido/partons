use serde::{Deserialize, Serialize};

// This should be i32, but unfortunately it is not honored by all sets:
// https://lhapdfsets.web.cern.ch/current/JAM20-SIDIS_FF_hadron_nlo/JAM20-SIDIS_FF_hadron_nlo.info
pub type PID = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    #[serde(rename = "SetDesc")]
    set_desc: String,
    #[serde(rename = "SetIndex")]
    set_index: u32,
    #[serde(rename = "Authors")]
    authors: String, // TODO: replace with sequence of strings
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
}
