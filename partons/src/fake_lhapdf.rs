#![cfg(not(feature = "lhapdf"))]

use super::noop::{Pdf as NoopPdf, PdfSet as NoopPdfSet};
use crate::{PdfEnum, PdfSetEnum, PdfUncertainty};

#[derive(Debug)]
pub struct Pdf(NoopPdf);

impl crate::Pdf for Pdf {
    fn alphas_q2(&self, q2: f64) -> f64 {
        self.0.alphas_q2(q2)
    }

    fn force_positive(&mut self) -> i32 {
        self.0.force_positive()
    }

    fn set(&self) -> crate::PdfSetEnum {
        self.0.set()
    }

    fn set_force_positive(&mut self, mode: i32) {
        self.0.set_force_positive(mode)
    }

    fn x_max(&self) -> f64 {
        self.0.x_max()
    }

    fn x_min(&self) -> f64 {
        self.0.x_min()
    }

    fn xfx_q2(&self, id: i32, x: f64, q2: f64) -> f64 {
        self.0.xfx_q2(id, x, q2)
    }
}

impl Pdf {
    pub fn new(_: &str) -> crate::Result<PdfEnum> {
        Ok(Self(NoopPdf {}).into())
    }
}

#[derive(Debug)]
pub struct PdfSet(NoopPdfSet);

impl crate::PdfSet for PdfSet {
    fn entry(&self, key: &str) -> Option<String> {
        self.0.entry(key)
    }

    fn error_type(&self) -> String {
        self.0.error_type()
    }

    fn pdfs(&self) -> crate::Result<Vec<PdfEnum>> {
        self.0.pdfs()
    }

    fn uncertainty(&self, values: &[f64], cl: f64, alternative: bool) -> PdfUncertainty {
        self.0.uncertainty(values, cl, alternative)
    }
}

impl PdfSet {
    pub fn available() -> Vec<String> {
        NoopPdfSet::available()
    }

    pub fn new(setname: &str) -> crate::Result<PdfSetEnum> {
        NoopPdfSet::new(setname)
    }
}
