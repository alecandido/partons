use super::noop::{Pdf as NoopPdf, PdfSet as NoopPdfSet};
use cfg_if::cfg_if;
use enum_dispatch::enum_dispatch;
use std::result;
use thiserror::Error;

cfg_if! {
    if #[cfg(feature = "lhapdf")] {
        use super::lhapdf::{Pdf as LhapdfPdf, PdfSet as LhapdfPdfSet};
    } else {
        use super::fake_lhapdf::{Pdf as LhapdfPdf, PdfSet as LhapdfPdfSet};
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Backend(#[from] anyhow::Error),
}

pub const CL_1_SIGMA: f64 = 68.268_949_213_708_58;

pub type Result<T> = result::Result<T, Error>;

pub struct PdfUncertainty {
    pub val: f64,
    pub pos: f64,
    pub neg: f64,
}

#[derive(Debug)]
#[enum_dispatch]
#[non_exhaustive]
pub enum PdfEnum {
    Noop(NoopPdf),
    Lhapdf(LhapdfPdf),
}

#[enum_dispatch(PdfEnum)]
pub trait Pdf {
    fn alphas_q2(&self, q2: f64) -> f64;

    fn force_positive(&mut self) -> i32;

    fn set(&self) -> PdfSetEnum;

    fn set_force_positive(&mut self, mode: i32);

    fn x_max(&self) -> f64;

    fn x_min(&self) -> f64;

    fn xfx_q2(&self, id: i32, x: f64, q2: f64) -> f64;
}

impl PdfEnum {
    pub fn new(pdfname: &str) -> Result<Self> {
        // TODO: implement the logic choosing the right backend
        LhapdfPdf::new(pdfname)
    }
}

#[derive(Debug)]
#[enum_dispatch]
#[non_exhaustive]
pub enum PdfSetEnum {
    Noop(NoopPdfSet),
    Lhapdf(LhapdfPdfSet),
}

#[enum_dispatch(PdfSetEnum)]
pub trait PdfSet {
    fn entry(&self, key: &str) -> Option<String>;

    fn error_type(&self) -> String;

    fn pdfs(&self) -> Result<Vec<PdfEnum>>;

    fn uncertainty(&self, values: &[f64], cl: f64, alternative: bool) -> PdfUncertainty;
}

impl PdfSetEnum {
    pub fn available() -> Vec<String> {
        LhapdfPdfSet::available()
    }

    pub fn new(setname: &str) -> Result<Self> {
        // TODO: implement the logic choosing the right backend
        LhapdfPdfSet::new(setname)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn available_pdf_sets() {
        let pdf_sets = PdfSetEnum::available();

        assert!(pdf_sets
            .iter()
            .any(|pdf_set| pdf_set == "NNPDF40_nnlo_as_01180"));
    }

    #[test]
    fn check_pdf() -> Result<()> {
        let pdf_0 = PdfEnum::new("NNPDF40_nnlo_as_01180")?;
        let pdf_1 = PdfEnum::new("331100")?;

        let value_0 = pdf_0.xfx_q2(2, 0.5, 90.0 * 90.0);
        let value_1 = pdf_1.xfx_q2(2, 0.5, 90.0 * 90.0);

        assert_ne!(value_0, 0.0);
        assert_eq!(value_0, value_1);

        let value_0 = pdf_0.alphas_q2(90.0 * 90.0);
        let value_1 = pdf_1.alphas_q2(90.0 * 90.0);

        assert_ne!(value_0, 0.0);
        assert_eq!(value_0, value_1);

        assert_eq!(
            PdfEnum::new("NNPDF40_nnlo_as_01180/10000")
                .unwrap_err()
                .to_string(),
            "PDF NNPDF40_nnlo_as_01180/10000 is out of the member range of set NNPDF40_nnlo_as_01180"
        );

        assert_eq!(
            PdfEnum::new("0").unwrap_err().to_string(),
            "Info file not found for PDF set ''"
        );

        assert_eq!(pdf_0.x_min(), 1e-9);
        assert_eq!(pdf_0.x_max(), 1.0);
        assert_eq!(pdf_1.x_min(), 1e-9);
        assert_eq!(pdf_1.x_max(), 1.0);

        Ok(())
    }

    #[test]
    fn check_setname_and_nmem() -> Result<()> {
        let pdf = PdfEnum::new("NNPDF40_nnlo_as_01180/1")?;

        let value = pdf.xfx_q2(2, 0.5, 90.0 * 90.0);

        assert_ne!(value, 0.0);

        let value = pdf.alphas_q2(90.0 * 90.0);

        assert_ne!(value, 0.0);

        assert_eq!(
            PdfEnum::new("foobar/0").unwrap_err().to_string(),
            "Info file not found for PDF set 'foobar'"
        );

        Ok(())
    }

    #[test]
    fn check_pdf_set() -> Result<()> {
        let pdf_set = PdfSetEnum::new("NNPDF40_nnlo_as_01180")?;

        assert!(matches!(pdf_set.entry("Particle"), Some(value) if value == "2212"));
        assert!(matches!(pdf_set.entry("Flavors"), Some(value)
            if value == "[-5, -4, -3, -2, -1, 21, 1, 2, 3, 4, 5]"));
        assert_eq!(pdf_set.entry("idontexist"), None);

        assert_eq!(pdf_set.error_type(), "replicas");

        assert_eq!(
            PdfSetEnum::new("IDontExist").unwrap_err().to_string(),
            "Info file not found for PDF set 'IDontExist'"
        );

        assert_eq!(pdf_set.pdfs().unwrap().len(), 101);

        let uncertainty = pdf_set.uncertainty(&[0.0; 101], CL_1_SIGMA, false);

        assert_eq!(uncertainty.val, 0.0);
        assert_eq!(uncertainty.pos, 0.0);
        assert_eq!(uncertainty.neg, 0.0);

        Ok(())
    }

    #[test]
    fn debug_pdf_set() -> Result<()> {
        let pdf_set = PdfSetEnum::new("NNPDF40_nnlo_as_01180")?;

        assert_eq!(format!("{:?}", pdf_set), "Lhapdf(lhapdf::PdfSet)");

        Ok(())
    }

    #[test]
    fn check_pdf_pdfset() -> Result<()> {
        let pdf_set0 = PdfSetEnum::new("NNPDF40_nnlo_as_01180")?;
        let pdf_set1 = PdfEnum::new("NNPDF40_nnlo_as_01180/0")?.set();

        assert_eq!(pdf_set0.entry("Particle"), pdf_set1.entry("Particle"));
        assert_eq!(pdf_set0.entry("NumMembers"), pdf_set1.entry("NumMembers"));

        Ok(())
    }

    #[test]
    fn force_positive() -> Result<()> {
        let mut pdf = PdfEnum::new("NNPDF40_nnlo_as_01180/1")?;

        assert_eq!(pdf.force_positive(), 0);

        pdf.set_force_positive(1);
        assert_eq!(pdf.force_positive(), 1);

        Ok(())
    }
}
