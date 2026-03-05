use crate::CastError;
use crate::PythonElement;
use crate::PythonElementCast;
use crate::PythonSet;
use crate::integer::PythonInteger;
use crate::integer::PythonIntegerSet;
use crate::rational::PythonRational;
use crate::rational::PythonRationalSet;
use crate::rational_polynomial::PythonRationalPolynomial;
use crate::rational_polynomial::PythonRationalPolynomialSet;
use algebraeon::nzq::Natural;
use algebraeon::rings::isolated_algebraic::PAdicAlgebraic;
use algebraeon::rings::isolated_algebraic::PAdicAlgebraicStructure;
use algebraeon::rings::isolated_algebraic::PAdicRational;
use algebraeon::rings::natural::factorization::primes::is_prime_nat;
use algebraeon::rings::valuation::Valuation;
use algebraeon::sets::structure::SetSignature;
use num_bigint::BigInt;
use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::types::PyList;
use pyo3::types::PyTuple;
use pyo3::{IntoPyObjectExt, prelude::*};

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PythonPAdicAlgebraicSet {
    p: Natural, // a prime
}

impl PythonPAdicAlgebraicSet {
    pub fn new(p: Natural) -> Option<Self> {
        if is_prime_nat(&p) {
            Some(Self { p })
        } else {
            None
        }
    }
}

impl PythonSet for PythonPAdicAlgebraicSet {
    type Elem = PythonPAdicAlgebraic;

    fn from_elem(&self, elem: PAdicAlgebraic) -> Self::Elem {
        PythonPAdicAlgebraic { inner: elem }
    }

    fn str(&self) -> String {
        format!("Alg(ℚ, ℚ{})", self.p)
    }

    fn repr(&self) -> String {
        format!("PAdicAlg({})", self.p)
    }
}

impl_pymethods_set!(PythonPAdicAlgebraicSet);

#[pymethods]
impl PythonPAdicAlgebraicSet {
    pub fn roots<'py>(&self, poly: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
        let py = poly.py();
        if let Ok(poly) = PythonRationalPolynomialSet::default().implicit_cast(poly) {
            Ok(PyList::new(
                py,
                poly.inner
                    .all_padic_roots(&self.p)
                    .into_iter()
                    .map(|root| PythonPAdicAlgebraic { inner: root })
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
                    .all_padic_roots(&self.p)
                    .into_iter()
                    .map(|root| PythonPAdicAlgebraic { inner: root })
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

#[pyclass(name = "PAdicAlg")]
#[derive(Debug, Clone)]
pub struct PythonPAdicAlgebraic {
    pub inner: PAdicAlgebraic,
}

impl PythonElement for PythonPAdicAlgebraic {
    type Set = PythonPAdicAlgebraicSet;

    type Structure = PAdicAlgebraicStructure;

    fn structure(&self) -> Self::Structure {
        PAdicAlgebraicStructure::new(self.inner.p().clone())
    }

    fn to_elem(&self) -> &<Self::Structure as SetSignature>::Set {
        &self.inner
    }

    fn into_elem(self) -> <Self::Structure as SetSignature>::Set {
        self.inner
    }

    fn set(&self) -> Self::Set {
        PythonPAdicAlgebraicSet {
            p: self.inner.p().clone(),
        }
    }

    fn str(&self) -> String {
        format!("{}", self.inner)
    }

    fn repr(&self) -> String {
        format!("PAdicAlg({})({})", self.inner.p(), self.inner)
    }
}

impl<'py> PythonElementCast<'py> for PythonPAdicAlgebraicSet {
    fn proper_subset_cast_impl(&self, obj: &Bound<'py, PyAny>) -> Result<Self::Elem, CastError> {
        if let Ok(obj) = PythonRationalSet::default().subset_cast_impl(obj) {
            return Ok(PythonPAdicAlgebraic {
                inner: PAdicAlgebraic::Rational(PAdicRational::from_rational(
                    self.p.clone(),
                    obj.inner,
                )),
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

impl_pymethods_elem!(PythonPAdicAlgebraic);
impl_pymethods_eq!(PythonPAdicAlgebraic);
impl_pymethods_pos!(PythonPAdicAlgebraic);
impl_pymethods_add!(PythonPAdicAlgebraic);
impl_pymethods_neg!(PythonPAdicAlgebraic);
impl_pymethods_sub!(PythonPAdicAlgebraic);
impl_pymethods_mul!(PythonPAdicAlgebraic);
impl_pymethods_div!(PythonPAdicAlgebraic);
impl_pymethods_int_pow!(PythonPAdicAlgebraic);

#[pymethods]
impl PythonPAdicAlgebraic {
    pub fn __int__<'py>(&self, py: Python<'py>) -> PyResult<BigInt> {
        Ok(PythonIntegerSet::default()
            .explicit_cast(&self.clone().into_bound_py_any(py)?)?
            .__int__())
    }

    pub fn is_rational(&self) -> bool {
        match &self.inner {
            PAdicAlgebraic::Rational(_) => true,
            PAdicAlgebraic::Algebraic(_) => false,
        }
    }

    pub fn minimal_polynomial(&self) -> PythonRationalPolynomial {
        PythonRationalPolynomial {
            inner: self.inner.min_poly(),
        }
    }

    pub fn isolate<'py>(&self, py: Python<'py>) -> Py<PyAny> {
        let ball = self.inner.isolating_ball();
        match ball.v {
            Valuation::Infinity => {
                // rational
                PythonRational { inner: ball.c }.into_py_any(py).unwrap()
            }
            Valuation::Finite(v) => {
                // irrational
                PyTuple::new(
                    py,
                    vec![
                        PythonRational { inner: ball.c }.into_py_any(py).unwrap(),
                        PythonInteger { inner: v }.into_py_any(py).unwrap(),
                    ],
                )
                .unwrap()
                .into_py_any(py)
                .unwrap()
            }
        }
    }
}
