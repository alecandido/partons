use serde::{Deserialize, Serialize};

// This should be i32, but unfortunately it is not honored by all sets:
// https://lhapdfsets.web.cern.ch/current/JAM20-SIDIS_FF_hadron_nlo/JAM20-SIDIS_FF_hadron_nlo.info
pub type PID = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    SetDesc: String,
    SetIndex: u32,
    Authors: String, // TODO: replace with sequence of strings
    #[serde(default)]
    Reference: Option<String>,
    #[serde(default)]
    Format: Option<String>, // TODO: replace with enum
    DataVersion: u32,
    NumMembers: u32,
    #[serde(default)]
    Particle: Option<PID>,
    Flavors: Vec<PID>,
    OrderQCD: u32,
    FlavorScheme: String, // TODO: replace with enum
    #[serde(default)]
    NumFlavors: Option<u32>,
    #[serde(default)]
    ErrorType: Option<String>, // TODO: replace with enum
    XMin: f64,
    XMax: f64,
    QMin: f64,
    QMax: f64,
    #[serde(default)]
    MZ: Option<f64>,
    #[serde(default)]
    MUp: Option<f64>,
    #[serde(default)]
    MDown: Option<f64>,
    #[serde(default)]
    MStrange: Option<f64>,
    #[serde(default)]
    MCharm: Option<f64>,
    #[serde(default)]
    MBottom: Option<f64>,
    #[serde(default)]
    MTop: Option<f64>,
    #[serde(default)]
    AlphaS_MZ: Option<f64>,
    #[serde(default)]
    AlphaS_OrderQCD: Option<u32>,
    #[serde(default)]
    AlphaS_Type: Option<String>, // TODO: replace with enum
    #[serde(default)]
    AlphaS_Qs: Option<Vec<f64>>,
    #[serde(default)]
    AlphaS_Vals: Option<Vec<f64>>,
    #[serde(default)]
    AlphaS_Lambda4: Option<f64>,
    #[serde(default)]
    AlphaS_Lambda5: Option<f64>,
    #[serde(default)]
    Extrapolator: Option<String>, // TODO: replace with enum
}
