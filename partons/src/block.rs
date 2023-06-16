//! Interpolation block
use std::collections::HashMap;

use anyhow::{anyhow, Result};
use ndarray::{Array1, Array3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub pids: Array1<i32>,
    pub xgrid: Array1<f64>,
    pub mu2grid: Array1<f64>,
    pub(crate) values: Array3<f64>,
    pid_map: HashMap<i32, usize>,
}

impl Block {
    pub(crate) fn new(
        pids: Array1<i32>,
        xgrid: Array1<f64>,
        mu2grid: Array1<f64>,
        values: Array3<f64>,
    ) -> Self {
        assert_eq!(values.shape(), &[pids.len(), xgrid.len(), mu2grid.len()]);

        let pid_map = pids.iter().enumerate().map(|(i, v)| (*v, i)).collect();

        Self {
            pids,
            xgrid,
            mu2grid,
            values,
            pid_map,
        }
    }

    pub(crate) fn pid_index(&self, pid: i32) -> Result<usize> {
        self.pid_map
            .get(&pid)
            .ok_or(anyhow!("PID '{}' not found", pid))
            .map(|idx| *idx)
    }

    // TODO: delegate interpolation to ndinterp
    pub(crate) fn interp(&self, pid: i32, x: f64, mu2: f64) -> Result<f64> {
        // determine interpolation slice
        let slice = self.pid_index(pid)?;

        let idx = self
            .xgrid
            .iter()
            .enumerate()
            .filter(|(_, y)| y > &&x)
            .next()
            .unwrap()
            .0;
        let idmu = self
            .mu2grid
            .iter()
            .enumerate()
            .filter(|(_, nu2)| nu2 > &&mu2)
            .next()
            .unwrap()
            .0;

        return Ok(self.values[[slice, idx, idmu]]);
    }
}
