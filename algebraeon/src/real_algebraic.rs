use crate::CastError;
use crate::PythonElement;
use crate::PythonElementCast;
use crate::PythonSet;
use crate::integer::PythonIntegerSet;
use crate::rational::PythonRational;
use crate::rational::PythonRationalSet;
use crate::rational_polynomial::PythonRationalPolynomial;
use crate::rational_polynomial::PythonRationalPolynomialSet;
use algebraeon::rings::isolated_algebraic::RealAlgebraic;
use algebraeon::rings::isolated_algebraic::RealAlgebraicCanonicalStructure;
use algebraeon::rings::isolated_algebraic::RealIsolatingRegion;
use algebraeon::sets::structure::MetaType;
use algebraeon::sets::structure::SetSignature;
use num_bigint::BigInt;
use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::types::PyList;
use pyo3::types::PyTuple;
use pyo3::{IntoPyObjectExt, prelude::*};

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PythonRealAlgebraicSet {}

impl PythonSet for PythonRealAlgebraicSet {
    type Elem = PythonRealAlgebraic;

    fn from_elem(&self, elem: RealAlgebraic) -> Self::Elem {
        PythonRealAlgebraic { inner: elem }
    }

    fn str(&self) -> String {
        "Alg(ℚ, ℝ)".to_string()
    }

    fn repr(&self) -> String {
        "RealAlg".to_string()
    }
}

impl_pymethods_set!(PythonRealAlgebraicSet);

#[pymethods]
impl PythonRealAlgebraicSet {
    pub fn roots<'py>(&self, poly: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
        let py = poly.py();
        if let Ok(poly) = PythonRationalPolynomialSet::default().implicit_cast(poly) {
            Ok(PyList::new(
                py,
                poly.inner
                    .all_real_roots()
                    .into_iter()
                    .map(|root| PythonRealAlgebraic { inner: root })
                    .collect::<Vec<_>>(),
            )
            .unwrap()
            .into_py_any(py)
            .unwrap())
        } else {
            todo!()
        }
    }

    pub fn distinct_roots<'py>(&self, poly: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
        let py = poly.py();
        if let Ok(poly) = PythonRationalPolynomialSet::default().implicit_cast(poly) {
            Ok(PyList::new(
                py,
                poly.inner
                    .primitive_squarefree_part()
                    .all_real_roots()
                    .into_iter()
                    .map(|root| PythonRealAlgebraic { inner: root })
                    .collect::<Vec<_>>(),
            )
            .unwrap()
            .into_py_any(py)
            .unwrap())
        } else {
            todo!()
        }
    }
}

#[pyclass(name = "RealAlg")]
#[derive(Debug, Clone)]
pub struct PythonRealAlgebraic {
    pub inner: RealAlgebraic,
}

impl PythonElement for PythonRealAlgebraic {
    type Set = PythonRealAlgebraicSet;

    type Structure = RealAlgebraicCanonicalStructure;

    fn structure(&self) -> Self::Structure {
        RealAlgebraic::structure()
    }

    fn to_elem(&self) -> &<Self::Structure as SetSignature>::Set {
        &self.inner
    }

    fn into_elem(self) -> <Self::Structure as SetSignature>::Set {
        self.inner
    }

    fn set(&self) -> Self::Set {
        PythonRealAlgebraicSet {}
    }

    fn str(&self) -> String {
        format!("{}", self.inner)
    }

    fn repr(&self) -> String {
        format!("RealAlg({})", self.inner)
    }
}

impl<'py> PythonElementCast<'py> for PythonRealAlgebraicSet {
    fn proper_subset_cast_impl(&self, obj: &Bound<'py, PyAny>) -> Result<Self::Elem, CastError> {
        if let Ok(obj) = PythonRationalSet::default().subset_cast_impl(obj) {
            return Ok(PythonRealAlgebraic {
                inner: RealAlgebraic::Rational(obj.inner),
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

impl_pymethods_elem!(PythonRealAlgebraic);
impl_pymethods_cmp!(PythonRealAlgebraic);
impl_pymethods_pos!(PythonRealAlgebraic);
impl_pymethods_add!(PythonRealAlgebraic);
impl_pymethods_neg!(PythonRealAlgebraic);
impl_pymethods_sub!(PythonRealAlgebraic);
impl_pymethods_mul!(PythonRealAlgebraic);
impl_pymethods_div!(PythonRealAlgebraic);
impl_pymethods_int_pow!(PythonRealAlgebraic);

#[pymethods]
impl PythonRealAlgebraic {
    pub fn __int__<'py>(&self, py: Python<'py>) -> PyResult<BigInt> {
        Ok(PythonIntegerSet::default()
            .explicit_cast(&self.clone().into_bound_py_any(py)?)?
            .__int__())
    }

    pub fn is_rational(&self) -> bool {
        match &self.inner {
            RealAlgebraic::Rational(_) => true,
            RealAlgebraic::Real(_) => false,
        }
    }

    pub fn minimal_polynomial(&self) -> PythonRationalPolynomial {
        PythonRationalPolynomial {
            inner: self.inner.min_poly(),
        }
    }

    pub fn isolate<'py>(&self, py: Python<'py>) -> Py<PyAny> {
        match self.inner.isolate() {
            RealIsolatingRegion::Rational(r) => {
                PythonRational { inner: r.clone() }.into_py_any(py).unwrap()
            }
            RealIsolatingRegion::Interval(a, b) => PyTuple::new(
                py,
                vec![
                    PythonRational { inner: a.clone() },
                    PythonRational { inner: b.clone() },
                ],
            )
            .unwrap()
            .into_py_any(py)
            .unwrap(),
        }
    }

    pub fn approximate<'py>(&mut self, r: &Bound<'py, PyAny>) -> PyResult<PythonRational> {
        let r = PythonRationalSet::default().implicit_cast(r)?;
        self.inner.refine_to_accuracy_mut(&r.inner);
        Ok(PythonRational {
            inner: self.inner.approximate(),
        })
    }
}
