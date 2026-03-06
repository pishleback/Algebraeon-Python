use crate::CastError;
use crate::PythonElement;
use crate::PythonElementCast;
use crate::PythonSet;
use crate::algebraeon_to_bignum_int;
use crate::integer::PythonIntegerSet;
use crate::real_algebraic::PythonRealAlgebraicSet;
use algebraeon::nzq::Integer;
use algebraeon::nzq::Rational;
use algebraeon::nzq::RationalCanonicalStructure;
use algebraeon::rings::isolated_algebraic::RealAlgebraic;
use algebraeon::sets::structure::MetaType;
use algebraeon::sets::structure::SetSignature;
use num_bigint::BigInt;
use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::{IntoPyObjectExt, exceptions::PyTypeError, prelude::*};

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PythonRationalSet {}

impl PythonSet for PythonRationalSet {
    type Elem = PythonRational;

    fn from_elem(&self, elem: Rational) -> Self::Elem {
        PythonRational { inner: elem }
    }

    fn str(&self) -> String {
        "ℚ".to_string()
    }

    fn repr(&self) -> String {
        "Rat".to_string()
    }
}

impl_pymethods_set!(PythonRationalSet);

#[pyclass(name = "Rat")]
#[derive(Debug, Clone)]
pub struct PythonRational {
    pub inner: Rational,
}

impl PythonElement for PythonRational {
    type Set = PythonRationalSet;

    type Structure = RationalCanonicalStructure;

    fn structure(&self) -> Self::Structure {
        Rational::structure()
    }

    fn to_elem(&self) -> &<Self::Structure as SetSignature>::Set {
        &self.inner
    }

    fn into_elem(self) -> <Self::Structure as SetSignature>::Set {
        self.inner
    }

    fn set(&self) -> Self::Set {
        PythonRationalSet {}
    }

    fn str(&self) -> String {
        format!("{}", self.inner)
    }

    fn repr(&self) -> String {
        format!("Rat({})", self.inner)
    }
}

impl<'py> PythonElementCast<'py> for PythonRationalSet {
    fn proper_subset_cast_impl(&self, obj: &Bound<'py, PyAny>) -> Result<Self::Elem, CastError> {
        if let Ok(obj) = PythonIntegerSet::default().subset_cast_impl(obj) {
            return Ok(PythonRational {
                inner: Rational::from(obj.to_elem()),
            });
        }
        Err(CastError::Type)
    }

    fn proper_supset_cast_impl(&self, obj: &Bound<'py, PyAny>) -> Result<Self::Elem, CastError> {
        if let Ok(obj) = PythonRealAlgebraicSet::default().supset_cast_impl(obj) {
            match obj.inner {
                RealAlgebraic::Rational(rational) => return Ok(PythonRational { inner: rational }),
                RealAlgebraic::Real(_) => {
                    return Err(CastError::Value);
                }
            }
        }
        Err(CastError::Type)
    }

    fn other_implicit_cast_impl(&self, obj: &Bound<'py, PyAny>) -> Result<Self::Elem, CastError> {
        let py = obj.py();
        if obj
            .get_type()
            .is(py.import("fractions").unwrap().getattr("Fraction").unwrap())
        {
            return Ok(PythonRational {
                inner: Rational::from_integers(
                    PythonIntegerSet::default()
                        .explicit_cast(&obj.getattr("numerator").unwrap())
                        .unwrap()
                        .to_elem(),
                    PythonIntegerSet::default()
                        .explicit_cast(&obj.getattr("denominator").unwrap())
                        .unwrap()
                        .to_elem(),
                ),
            });
        }
        Err(CastError::Type)
    }

    fn other_explicit_cast_impl(&self, _obj: &Bound<'py, PyAny>) -> Result<Self::Elem, CastError> {
        Err(CastError::Type)
    }
}

impl_pymethods_elem!(PythonRational);
impl_pymethods_cmp!(PythonRational);
impl_pymethods_pos!(PythonRational);
impl_pymethods_add!(PythonRational);
impl_pymethods_neg!(PythonRational);
impl_pymethods_sub!(PythonRational);
impl_pymethods_mul!(PythonRational);
impl_pymethods_div!(PythonRational);
impl_pymethods_int_pow!(PythonRational);

#[pymethods]
impl PythonRational {
    #[new]
    #[pyo3(signature = (obj1, obj2=None))]
    pub fn py_new<'py>(
        obj1: &Bound<'py, PyAny>,
        obj2: Option<&Bound<'py, PyAny>>,
    ) -> PyResult<Self> {
        let py = obj1.py();
        if let Some(obj2) = obj2 {
            if let Ok(obj1) = PythonIntegerSet::default().implicit_cast(obj1)
                && let Ok(obj2) = PythonIntegerSet::default().implicit_cast(obj2)
            {
                Ok(Self::py_new(obj1.into_py_any(py)?.bind(py), None)?
                    .__truediv__(
                        Self::py_new(obj2.into_py_any(py)?.bind(py), None)?
                            .into_py_any(py)?
                            .bind(py),
                    )?
                    .extract::<Self>(py)
                    .unwrap())
            } else {
                Err(PyTypeError::new_err(format!(
                    "expected integers for both argument but got `{}` and `{}`",
                    obj1.repr()?,
                    obj2.repr()?
                )))
            }
        } else {
            PythonRationalSet::default().explicit_cast(obj1)
        }
    }

    pub fn __int__(&self) -> PyResult<BigInt> {
        if let Ok(n) = Integer::try_from(&self.inner) {
            Ok(algebraeon_to_bignum_int(&n))
        } else {
            Err(PyValueError::new_err(""))
        }
    }
}
