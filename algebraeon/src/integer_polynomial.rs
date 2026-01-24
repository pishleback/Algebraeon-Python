use crate::PythonElement;
use crate::PythonElementCast;
use crate::PythonPolynomialSet;
use crate::PythonSet;
use crate::PythonToPolynomialSet;
use crate::integer::PythonIntegerSet;
use algebraeon::nzq::Integer;
use algebraeon::nzq::IntegerCanonicalStructure;
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
pub struct PythonIntegerPolynomialSet {}

impl PythonSet for PythonIntegerPolynomialSet {
    type Elem = PythonIntegerPolynomial;

    fn from_elem(&self, elem: Polynomial<Integer>) -> Self::Elem {
        PythonIntegerPolynomial { inner: elem }
    }

    fn str(&self) -> String {
        format!("{}[λ]", PythonIntegerSet::default().str())
    }

    fn repr(&self) -> String {
        format!("Polynomial({})", PythonIntegerSet::default().repr())
    }
}

impl PythonPolynomialSet for PythonIntegerPolynomialSet {
    fn var(&self) -> <Self as PythonSet>::Elem {
        PythonIntegerPolynomial {
            inner: Polynomial::var(),
        }
    }
}

impl_pymethods_set!(PythonIntegerPolynomialSet);
impl_pymethods_polynomial_set!(PythonIntegerPolynomialSet);

impl PythonToPolynomialSet for PythonIntegerSet {
    type PolynomialSet = PythonIntegerPolynomialSet;

    fn polynomials(&self) -> Self::PolynomialSet {
        PythonIntegerPolynomialSet::default()
    }
}

impl_pymethods_to_polynomial_set!(PythonIntegerSet);

#[pyclass()]
#[derive(Debug, Clone)]
pub struct PythonIntegerPolynomial {
    pub inner: Polynomial<Integer>,
}

impl PythonElement for PythonIntegerPolynomial {
    type Set = PythonIntegerPolynomialSet;

    type Structure = PolynomialStructure<IntegerCanonicalStructure, IntegerCanonicalStructure>;

    fn structure(&self) -> Self::Structure {
        Integer::structure().into_polynomials()
    }

    fn to_elem(&self) -> &<Self::Structure as SetSignature>::Set {
        &self.inner
    }

    fn into_elem(self) -> <Self::Structure as SetSignature>::Set {
        self.inner
    }

    fn set(&self) -> Self::Set {
        PythonIntegerPolynomialSet {}
    }

    fn str(&self) -> String {
        format!("{}", self.inner)
    }

    fn repr(&self) -> String {
        format!(
            "Polynomial({}, {})",
            self.inner,
            PythonIntegerSet::default().repr()
        )
    }
}

impl<'py> PythonElementCast<'py> for PythonIntegerPolynomialSet {
    fn cast_exact(&self, obj: &Bound<'py, PyAny>) -> Option<Self::Elem> {
        obj.extract::<Self::Elem>().ok()
    }

    fn cast_equiv(&self, _obj: &Bound<'py, PyAny>) -> PyResult<PythonIntegerPolynomial> {
        Err(PyTypeError::new_err(""))
    }

    fn cast_proper_subtype(&self, obj: &Bound<'py, PyAny>) -> Option<PythonIntegerPolynomial> {
        if let Ok(n) = PythonIntegerSet::default().cast_subtype(obj) {
            Some(PythonIntegerPolynomial {
                inner: Polynomial::constant(n.to_elem().clone()),
            })
        } else {
            None
        }
    }
}

impl_pymethods_elem!(PythonIntegerPolynomial);
impl_pymethods_eq!(PythonIntegerPolynomial);
impl_pymethods_pos!(PythonIntegerPolynomial);
impl_pymethods_add!(PythonIntegerPolynomial);
impl_pymethods_neg!(PythonIntegerPolynomial);
impl_pymethods_sub!(PythonIntegerPolynomial);
impl_pymethods_mul!(PythonIntegerPolynomial);
impl_pymethods_div!(PythonIntegerPolynomial);
impl_pymethods_nat_pow!(PythonIntegerPolynomial);

#[pymethods]
impl PythonIntegerPolynomial {
    #[new]
    pub fn py_new<'py>(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        PythonIntegerPolynomialSet::default().cast_subtype(obj)
    }
}
