use crate::CastError;
use crate::PythonElement;
use crate::PythonElementCast;
use crate::PythonPolynomialSet;
use crate::PythonSet;
use crate::PythonToPolynomialSet;
use crate::integer_polynomial::PythonIntegerPolynomialSet;
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
use pyo3::{IntoPyObjectExt, prelude::*};

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PythonRationalPolynomialSet {}

impl PythonSet for PythonRationalPolynomialSet {
    type Elem = PythonRationalPolynomial;

    fn from_elem(&self, elem: Polynomial<Rational>) -> Self::Elem {
        PythonRationalPolynomial { inner: elem }
    }

    fn str(&self) -> String {
        format!("{}[λ]", PythonRationalSet::default().str())
    }

    fn repr(&self) -> String {
        format!("Poly({})", PythonRationalSet::default().repr())
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

#[pyclass(name = "RatPoly")]
#[derive(Debug, Clone)]
pub struct PythonRationalPolynomial {
    pub inner: Polynomial<Rational>,
}

impl PythonElement for PythonRationalPolynomial {
    type Set = PythonRationalPolynomialSet;

    type Structure = PolynomialStructure<RationalCanonicalStructure, RationalCanonicalStructure>;

    fn structure(&self) -> Self::Structure {
        Rational::structure().into_polynomials()
    }

    fn to_elem(&self) -> &<Self::Structure as SetSignature>::Set {
        &self.inner
    }

    fn into_elem(self) -> <Self::Structure as SetSignature>::Set {
        self.inner
    }

    fn set(&self) -> Self::Set {
        PythonRationalPolynomialSet {}
    }

    fn str(&self) -> String {
        format!("{}", self.inner)
    }

    fn repr(&self) -> String {
        format!(
            "Poly({}, {})",
            self.inner,
            PythonRationalSet::default().repr()
        )
    }
}

impl<'py> PythonElementCast<'py> for PythonRationalPolynomialSet {
    fn proper_subset_cast_impl(&self, obj: &Bound<'py, PyAny>) -> Result<Self::Elem, CastError> {
        if let Ok(n) = PythonRationalSet::default().implicit_cast(obj) {
            return Ok(PythonRationalPolynomial {
                inner: Polynomial::constant(n.to_elem().clone()),
            });
        }
        if let Ok(p) = PythonIntegerPolynomialSet::default().implicit_cast(obj) {
            return Ok(PythonRationalPolynomial {
                inner: p.into_elem().apply_map_into(Rational::from),
            });
        }
        Err(CastError::Type)
    }

    fn proper_supset_cast_impl(&self, _obj: &Bound<'py, PyAny>) -> Result<Self::Elem, CastError> {
        Err(CastError::Type)
    }

    fn other_implicit_cast_impl(&self, _obj: &Bound<'py, PyAny>) -> Result<Self::Elem, CastError> {
        Err(CastError::Type)
    }

    fn other_explicit_cast_impl(&self, _obj: &Bound<'py, PyAny>) -> Result<Self::Elem, CastError> {
        Err(CastError::Type)
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
        PythonRationalPolynomialSet::default().explicit_cast(obj)
    }
}
