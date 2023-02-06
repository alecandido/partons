use crate::{PdfEnum, PdfSet as _, PdfSetEnum};
use anyhow::anyhow;
use cxx::{let_cxx_string, UniquePtr};
use std::fmt::{self, Debug, Formatter};

#[cxx::bridge]
mod ffi {
    // The type `PdfUncertainty` must be separate from the one defined in the C++ namespace LHAPDF
    // because it differs (at least) from LHAPDF 6.4.x to 6.5.x
    /// Structure for storage of uncertainty info calculated over a PDF error set.
    pub struct PdfUncertainty {
        /// The central value.
        pub central: f64,
        /// The unsymmetric error in positive direction.
        pub errplus: f64,
        /// The unsymmetric error in negative direction.
        pub errminus: f64,
        /// The symmetric error.
        pub errsymm: f64,
        /// The scale factor needed to convert between the PDF set's default confidence level and
        /// the requested confidence level.
        pub scale: f64,
        /// Extra variable for separate PDF and parameter variation errors with combined sets.
        pub errplus_pdf: f64,
        /// Extra variable for separate PDF and parameter variation errors with combined sets.
        pub errminus_pdf: f64,
        /// Extra variable for separate PDF and parameter variation errors with combined sets.
        pub errsymm_pdf: f64,
        /// Extra variable for separate PDF and parameter variation errors with combined sets.
        pub err_par: f64,
    }

    #[namespace = "LHAPDF"]
    unsafe extern "C++" {
        include!("LHAPDF/LHAPDF.h");

        fn availablePDFSets() -> &'static CxxVector<CxxString>;
        fn setVerbosity(verbosity: i32);

        type PDF;

        fn alphasQ2(self: &PDF, q2: f64) -> Result<f64>;
        fn forcePositive(self: &PDF) -> i32;
        fn setForcePositive(self: Pin<&mut PDF>, mode: i32);
        fn xfxQ2(self: &PDF, id: i32, x: f64, q2: f64) -> Result<f64>;

        type PDFSet;

        fn has_key(self: &PDFSet, key: &CxxString) -> bool;
        fn get_entry(self: &PDFSet, key: &CxxString) -> &'static CxxString;
        fn size(self: &PDFSet) -> usize;
    }

    unsafe extern "C++" {
        include!("partons/include/wrappers.hpp");

        fn get_pdfset_error_type(set: &PDFSet, setname: Pin<&mut CxxString>);

        fn pdf_with_lhaid(lhaid: i32) -> Result<UniquePtr<PDF>>;
        fn pdf_with_setname_and_nmem(setname: &CxxString) -> Result<UniquePtr<PDF>>;
        fn pdf_with_set_and_member(set: &PDFSet, member: i32) -> Result<UniquePtr<PDF>>;
        fn pdfset_new(setname: &CxxString) -> Result<UniquePtr<PDFSet>>;
        fn pdfset_from_pdf(pdf: &PDF) -> UniquePtr<PDFSet>;

        fn pdf_uncertainty(
            pdfset: &PDFSet,
            values: &[f64],
            cl: f64,
            alternative: bool,
        ) -> Result<PdfUncertainty>;
    }
}

pub struct Pdf {
    ptr: UniquePtr<ffi::PDF>,
}

impl Debug for Pdf {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // TODO: add more information if possible
        f.debug_struct("lhapdf::Pdf").finish()
    }
}

impl crate::Pdf for Pdf {
    fn alphas_q2(&self, q2: f64) -> f64 {
        self.ptr.alphasQ2(q2).unwrap()
    }

    fn force_positive(&mut self) -> i32 {
        self.ptr.pin_mut().forcePositive()
    }

    fn set(&self) -> crate::PdfSetEnum {
        PdfSet {
            ptr: ffi::pdfset_from_pdf(&self.ptr),
        }
        .into()
    }

    fn set_force_positive(&mut self, mode: i32) {
        self.ptr.pin_mut().setForcePositive(mode);
    }

    fn x_max(&self) -> f64 {
        // we can't use the member function from `PDF` because it's not marked as `const`
        self.set().entry("XMax").unwrap().parse().unwrap()
    }

    fn x_min(&self) -> f64 {
        // we can't use the member function from `PDF` because it's not marked as `const`
        self.set().entry("XMin").unwrap().parse().unwrap()
    }

    fn xfx_q2(&self, id: i32, x: f64, q2: f64) -> f64 {
        self.ptr.xfxQ2(id, x, q2).unwrap()
    }
}

impl Pdf {
    pub fn new(pdfname: &str) -> crate::Result<PdfEnum> {
        // suppress output
        ffi::setVerbosity(0);

        pdfname
            .parse::<i32>()
            .map_or_else(
                |_| {
                    let_cxx_string!(cxx_setname = pdfname.to_string());
                    ffi::pdf_with_setname_and_nmem(&cxx_setname)
                },
                |lhaid| ffi::pdf_with_lhaid(lhaid),
            )
            .map(|ptr| Self { ptr }.into())
            .map_err(|exc| crate::Error::Backend(anyhow!(exc)))
    }
}

pub struct PdfSet {
    ptr: UniquePtr<ffi::PDFSet>,
}

impl Debug for PdfSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // TODO: add more information if possible
        f.debug_struct("lhapdf::PdfSet").finish()
    }
}

impl crate::PdfSet for PdfSet {
    fn entry(&self, key: &str) -> Option<String> {
        let_cxx_string!(cxx_key = key);

        if self.ptr.has_key(&cxx_key) {
            Some(self.ptr.get_entry(&cxx_key).to_string_lossy().into_owned())
        } else {
            None
        }
    }

    fn error_type(&self) -> String {
        let_cxx_string!(string = "");

        ffi::get_pdfset_error_type(&self.ptr, string.as_mut());
        string.to_string_lossy().into_owned()
    }

    fn pdfs(&self) -> crate::Result<Vec<PdfEnum>> {
        (0..i32::try_from(self.ptr.size()).unwrap_or_else(|_| unreachable!()))
            .map(|member| {
                ffi::pdf_with_set_and_member(&self.ptr, member)
                    .map(|ptr| Pdf { ptr }.into())
                    .map_err(|exc| crate::Error::Backend(anyhow!(exc)))
            })
            .collect()
    }

    fn uncertainty(&self, values: &[f64], cl: f64, alternative: bool) -> crate::PdfUncertainty {
        let ffi::PdfUncertainty {
            central,
            errplus,
            errminus,
            ..
        } = ffi::pdf_uncertainty(&self.ptr, values, cl, alternative).unwrap();

        crate::PdfUncertainty {
            val: central,
            pos: errplus,
            neg: errminus,
        }
    }
}

impl PdfSet {
    pub fn available() -> Vec<String> {
        // suppress output
        ffi::setVerbosity(0);

        ffi::availablePDFSets()
            .iter()
            .map(|s| s.to_string_lossy().into_owned())
            .collect()
    }

    pub fn new(setname: &str) -> crate::Result<PdfSetEnum> {
        let_cxx_string!(cxx_setname = setname);

        // suppress output
        ffi::setVerbosity(0);

        ffi::pdfset_new(&cxx_setname)
            .map(|ptr| Self { ptr }.into())
            .map_err(|exc| crate::Error::Backend(anyhow!(exc)))
    }
}
