use anyhow::{anyhow, Result};
use ndarray::{array, Array1, Array3};

use std::{collections::HashMap, fmt::format};

pub struct Block {
    pub pids: Array1<i32>,
    pub xgrid: Array1<f64>,
    pub mugrid: Array1<f64>,
    values: Array3<f64>,
}

impl Block {
    pub fn new(
        pids: Array1<i32>,
        xgrid: Array1<f64>,
        mugrid: Array1<f64>,
        values: Array3<f64>,
    ) -> Self {
        Self {
            pids,
            xgrid,
            mugrid,
            values,
        }
    }

    pub fn pids_indices(&self, pids: &[i32]) -> Result<Vec<usize>> {
        let self_pids_map: HashMap<_, _> =
            self.pids.iter().enumerate().map(|(i, v)| (v, i)).collect();

        pids.iter()
            .map(|pid| {
                self_pids_map
                    .get(pid)
                    .map(|p| *p)
                    .ok_or(anyhow!("PID '{}' not found", pid))
            })
            .collect()
    }

    pub fn interp(&self, pids: &[i32], xs: &[f64], mus: &[f64]) -> Result<Array3<f64>> {
        let slices = self.pids_indices(pids)?;

        return Ok(array![[[0.]]]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block() -> Block {
        Block::new(
            array![-1, 1, 21],
            array![0.1, 0.5, 1.],
            array![10., 100.],
            array![[[0.]]],
        )
    }

    #[test]
    fn new_test() {
        block();
    }

    #[test]
    fn pids_indices_test() {
        let b = block();

        assert_eq!(
            b.pids_indices(&[1, 21, -1, 21, 21]).unwrap(),
            vec![1, 2, 0, 2, 2]
        );
    }

    #[test]
    fn interp_test() {
        ()
    }
}
