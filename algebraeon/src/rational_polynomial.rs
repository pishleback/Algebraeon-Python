use crate::PythonElement;
use crate::PythonElementCast;
use crate::PythonPolynomialSet;
use crate::PythonSet;
use crate::PythonStructure;
use crate::PythonToPolynomialSet;
use crate::integer_polynomial::PythonIntegerPolynomial;
use crate::rational::PythonRational;
use crate::rational::PythonRationalSet;
use algebraeon::nzq::Rational;
use algebraeon::nzq::RationalCanonicalStructure;
use algebraeon::rings::polynomial::Polynomial;
use algebraeon::rings::polynomial::PolynomialStructure;
use algebraeon::rings::polynomial::ToPolynomialSignature;
use algebraeon::sets::structure::MetaType;
use algebraeon::sets::structure::SetSignature;
use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::{IntoPyObjectExt, exceptions::PyTypeError, prelude::*};

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PythonRationalPolynomialSet {}

impl PythonSet for PythonRationalPolynomialSet {
    type Elem = PythonRationalPolynomial;

    fn str(&self) -> String {
        format!("{}[λ]", PythonRationalSet::default().str())
    }

    fn repr(&self) -> String {
        format!("Polynomial({})", PythonRationalSet::default().repr())
    }
}

impl PythonPolynomialSet for PythonRationalPolynomialSet {
    fn var(&self) -> <Self as PythonSet>::Elem {
        PythonRationalPolynomial {
            inner: Polynomial::var(),
        }
    }
}

impl_pymethods_set!(PythonRationalPolynomialSet);
impl_pymethods_polynomial_set!(PythonRationalPolynomialSet);

impl PythonToPolynomialSet for PythonRationalSet {
    type PolynomialSet = PythonRationalPolynomialSet;

    fn polynomials(&self) -> Self::PolynomialSet {
        PythonRationalPolynomialSet::default()
    }
}

impl_pymethods_to_polynomial_set!(PythonRationalSet);

#[pyclass]
#[derive(Debug, Clone)]
pub struct PythonRationalPolynomial {
    inner: Polynomial<Rational>,
}

impl PythonElement for PythonRationalPolynomial {
    type Set = PythonRationalPolynomialSet;

    fn set(&self) -> Self::Set {
        PythonRationalPolynomialSet {}
    }

    fn str(&self) -> String {
        format!("{}", self.inner)
    }

    fn repr(&self) -> String {
        format!(
            "Polynomial({}, {})",
            self.inner,
            PythonRationalSet::default().repr()
        )
    }
}

impl<'py> PythonElementCast<'py> for PythonRationalPolynomial {
    fn cast_equiv(_obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Err(PyTypeError::new_err(""))
    }

    fn cast_proper_subtype(obj: &Bound<'py, PyAny>) -> Option<Self> {
        if let Ok(n) = PythonRational::cast_subtype(obj) {
            Some(Self {
                inner: Polynomial::constant(n.inner().clone()),
            })
        } else if let Ok(p) = PythonIntegerPolynomial::cast_subtype(obj) {
            Some(Self {
                inner: p.into_inner().apply_map_into(Rational::from),
            })
        } else {
            None
        }
    }
}

impl PythonStructure for PythonRationalPolynomial {
    type Structure = PolynomialStructure<RationalCanonicalStructure, RationalCanonicalStructure>;

    fn structure(&self) -> Self::Structure {
        Rational::structure().into_polynomials()
    }

    fn inner(&self) -> &<Self::Structure as SetSignature>::Set {
        &self.inner
    }

    fn into_inner(self) -> <Self::Structure as SetSignature>::Set {
        self.inner
    }
}

impl_pymethods_elem!(PythonRationalPolynomial);
impl_pymethods_eq!(PythonRationalPolynomial);
impl_pymethods_pos!(PythonRationalPolynomial);
impl_pymethods_add!(PythonRationalPolynomial);
impl_pymethods_neg!(PythonRationalPolynomial);
impl_pymethods_sub!(PythonRationalPolynomial);
impl_pymethods_mul!(PythonRationalPolynomial);
impl_pymethods_div!(PythonRationalPolynomial);
impl_pymethods_nat_pow!(PythonRationalPolynomial);

#[pymethods]
impl PythonRationalPolynomial {
    #[new]
    pub fn py_new<'py>(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Self::cast_subtype(obj)
    }
}
