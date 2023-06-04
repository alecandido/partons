use super::interface::{self, PdfEnum, PdfSetEnum, PdfUncertainty};

#[derive(Debug)]
pub struct Pdf {}

impl interface::Pdf for Pdf {
    fn alphas_q2(&self, _: f64) -> f64 {
        1.0
    }

    fn force_positive(&mut self) -> i32 {
        1
    }

    fn set(&self) -> interface::PdfSetEnum {
        PdfSet {}.into()
    }

    fn set_force_positive(&mut self, _: i32) {}

    fn x_max(&self) -> f64 {
        1.0
    }

    fn x_min(&self) -> f64 {
        0.0
    }

    fn xfx_q2(&self, _: i32, x: f64, _: f64) -> f64 {
        x
    }
}

impl Pdf {
    pub fn new(_: &str) -> interface::Result<PdfEnum> {
        Ok(Pdf {}.into())
    }
}

#[derive(Debug)]
pub struct PdfSet {}

impl interface::PdfSet for PdfSet {
    fn entry(&self, _: &str) -> Option<String> {
        None
    }

    fn error_type(&self) -> String {
        "".to_string()
    }

    fn pdfs(&self) -> interface::Result<Vec<PdfEnum>> {
        Ok(vec![Pdf {}.into()])
    }

    fn uncertainty(&self, _: &[f64], _: f64, _: bool) -> PdfUncertainty {
        PdfUncertainty {
            val: 0.0,
            pos: 0.0,
            neg: 0.0,
        }
    }
}

impl PdfSet {
    pub fn available() -> Vec<String> {
        vec!["NOOP".to_string()]
    }

    pub fn new(_: &str) -> interface::Result<PdfSetEnum> {
        Ok(PdfSet {}.into())
    }
}
