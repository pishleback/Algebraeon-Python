use crate::CastError;
use crate::PythonElement;
use crate::PythonElementCast;
use crate::PythonSet;
use crate::integer::PythonIntegerSet;
use crate::rational::PythonRational;
use crate::rational_polynomial::PythonRationalPolynomial;
use crate::rational_polynomial::PythonRationalPolynomialSet;
use crate::real_algebraic::PythonRealAlgebraicSet;
use algebraeon::rings::isolated_algebraic::ComplexAlgebraic;
use algebraeon::rings::isolated_algebraic::ComplexAlgebraicCanonicalStructure;
use algebraeon::rings::isolated_algebraic::ComplexIsolatingRegion;
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
pub struct PythonComplexAlgebraicSet {}

impl PythonSet for PythonComplexAlgebraicSet {
    type Elem = PythonComplexAlgebraic;

    fn from_elem(&self, elem: ComplexAlgebraic) -> Self::Elem {
        PythonComplexAlgebraic { inner: elem }
    }

    fn str(&self) -> String {
        "Alg(ℚ, ℂ)".to_string()
    }

    fn repr(&self) -> String {
        "CpxAlg".to_string()
    }
}

impl_pymethods_set!(PythonComplexAlgebraicSet);

#[pymethods]
impl PythonComplexAlgebraicSet {
    pub fn roots<'py>(&self, poly: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
        let py = poly.py();
        if let Ok(poly) = PythonRationalPolynomialSet::default().implicit_cast(poly) {
            Ok(PyList::new(
                py,
                poly.inner
                    .all_complex_roots()
                    .into_iter()
                    .map(|root| PythonComplexAlgebraic { inner: root })
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
                    .all_complex_roots()
                    .into_iter()
                    .map(|root| PythonComplexAlgebraic { inner: root })
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

#[pyclass(name = "CpxAlg")]
#[derive(Debug, Clone)]
pub struct PythonComplexAlgebraic {
    pub inner: ComplexAlgebraic,
}

impl PythonElement for PythonComplexAlgebraic {
    type Set = PythonComplexAlgebraicSet;

    type Structure = ComplexAlgebraicCanonicalStructure;

    fn structure(&self) -> Self::Structure {
        ComplexAlgebraic::structure()
    }

    fn to_elem(&self) -> &<Self::Structure as SetSignature>::Set {
        &self.inner
    }

    fn into_elem(self) -> <Self::Structure as SetSignature>::Set {
        self.inner
    }

    fn set(&self) -> Self::Set {
        PythonComplexAlgebraicSet {}
    }

    fn str(&self) -> String {
        format!("{}", self.inner)
    }

    fn repr(&self) -> String {
        format!("RealAlg({})", self.inner)
    }
}

impl<'py> PythonElementCast<'py> for PythonComplexAlgebraicSet {
    fn proper_subset_cast_impl(&self, obj: &Bound<'py, PyAny>) -> Result<Self::Elem, CastError> {
        if let Ok(obj) = PythonRealAlgebraicSet::default().subset_cast_impl(obj) {
            return Ok(PythonComplexAlgebraic {
                inner: ComplexAlgebraic::Real(obj.inner),
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

impl_pymethods_elem!(PythonComplexAlgebraic);
impl_pymethods_eq!(PythonComplexAlgebraic);
impl_pymethods_pos!(PythonComplexAlgebraic);
impl_pymethods_add!(PythonComplexAlgebraic);
impl_pymethods_neg!(PythonComplexAlgebraic);
impl_pymethods_sub!(PythonComplexAlgebraic);
impl_pymethods_mul!(PythonComplexAlgebraic);
impl_pymethods_div!(PythonComplexAlgebraic);
impl_pymethods_int_pow!(PythonComplexAlgebraic);

#[pymethods]
impl PythonComplexAlgebraic {
    pub fn __int__<'py>(&self, py: Python<'py>) -> PyResult<BigInt> {
        Ok(PythonIntegerSet::default()
            .explicit_cast(&self.clone().into_bound_py_any(py)?)?
            .__int__())
    }

    pub fn is_rational(&self) -> bool {
        match &self.inner {
            ComplexAlgebraic::Real(real_algebraic) => match real_algebraic {
                algebraeon::rings::isolated_algebraic::RealAlgebraic::Rational(_) => true,
                algebraeon::rings::isolated_algebraic::RealAlgebraic::Real(_) => false,
            },
            ComplexAlgebraic::Complex(_) => false,
        }
    }

    pub fn is_real(&self) -> bool {
        match &self.inner {
            ComplexAlgebraic::Real(_) => true,
            ComplexAlgebraic::Complex(_) => false,
        }
    }

    pub fn minimal_polynomial(&self) -> PythonRationalPolynomial {
        PythonRationalPolynomial {
            inner: self.inner.min_poly(),
        }
    }

    pub fn isolate<'py>(&self, py: Python<'py>) -> Py<PyAny> {
        match self.inner.isolate() {
            ComplexIsolatingRegion::Rational(r) => {
                PythonRational { inner: r.clone() }.into_py_any(py).unwrap()
            }
            ComplexIsolatingRegion::RealInterval(a, b) => PyTuple::new(
                py,
                vec![
                    PythonRational { inner: a.clone() },
                    PythonRational { inner: b.clone() },
                ],
            )
            .unwrap()
            .into_py_any(py)
            .unwrap(),
            ComplexIsolatingRegion::Box(a, b, c, d) => PyTuple::new(
                py,
                vec![
                    PyTuple::new(
                        py,
                        vec![
                            PythonRational { inner: a.clone() },
                            PythonRational { inner: b.clone() },
                        ],
                    )
                    .unwrap(),
                    PyTuple::new(
                        py,
                        vec![
                            PythonRational { inner: c.clone() },
                            PythonRational { inner: d.clone() },
                        ],
                    )
                    .unwrap(),
                ],
            )
            .unwrap()
            .into_py_any(py)
            .unwrap(),
        }
    }
}
