//! Interpolation block
use anyhow::Result;
use ndarray::{Array1, Array2};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block1 {
    pub coords: Array1<f64>,
    pub(crate) values: Array1<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block2 {
    pub coords: (Array1<f64>, Array1<f64>),
    pub(crate) values: Array2<f64>,
}

impl Block2 {
    pub(crate) fn new(coords: (Array1<f64>, Array1<f64>), values: Array2<f64>) -> Self {
        assert_eq!(values.shape(), &[coords.0.len(), coords.1.len()]);

        Self { coords, values }
    }

    // TODO: delegate interpolation to ndinterp
    pub(crate) fn interp(&self, x1: f64, x2: f64) -> Result<f64> {
        return Ok(self.values[[0, 0]]);
    }
}
