use anyhow::{anyhow, Result};
use itertools::izip;
use ndarray::{Array1, Array3};

use std::collections::HashMap;

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
        assert_eq!(values.shape(), &[pids.len(), xgrid.len(), mugrid.len()]);

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

    pub fn interp(&self, pids: &[i32], xs: &[f64], mus: &[f64]) -> Result<Array1<f64>> {
        // determine interpolation slices, one per pid
        let slices = self.pids_indices(pids)?;

        let mut values = Vec::new();

        for (slice, _x, _mu) in izip!(slices, xs, mus) {
            let idx = 0;
            let idmu = 0;
            values.push(self.values[[slice, idx, idmu]]);
        }

        return Ok(Array1::from(values));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ndarray::array;

    fn block() -> Block {
        Block::new(
            array![-1, 1, 21],
            array![0.01, 0.1, 0.5, 1.],
            array![10., 100.],
            Array3::from_shape_fn((3, 4, 2), |(i, j, k)| (100 * i + 10 * j + k) as f64),
        )
    }

    #[test]
    fn new_test() {
        let b = block();

        fn sumn(n: u32) -> u32 {
            (0..n).sum()
        }

        assert_eq!(
            b.values.sum() as u32,
            (100 * sumn(3) * 4 * 2 + 10 * 3 * sumn(4) * 2 + 1 * 3 * 4 * sumn(2))
        )
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
        let b = block();

        let values = b
            .interp(
                &[1, 21, 1, -1],
                &[0.2, 0.2, 0.7, 0.7],
                &[15., 25., 35., 45.],
            )
            .unwrap();
        assert_eq!(&values.to_vec(), &[100., 200., 100., 0.])
    }
}
