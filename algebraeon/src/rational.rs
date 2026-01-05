use crate::PythonCast;
use crate::PythonStructure;
use crate::algebraeon_to_bignum_int;
use crate::impl_pymethods_add;
use crate::impl_pymethods_cmp;
use crate::impl_pymethods_div;
use crate::impl_pymethods_mul;
use crate::impl_pymethods_nat_pow;
use crate::impl_pymethods_neg;
use crate::impl_pymethods_pos;
use crate::impl_pymethods_sub;
use crate::integer::PythonInteger;
use algebraeon::nzq::Integer;
use algebraeon::nzq::Rational;
use algebraeon::nzq::RationalCanonicalStructure;
use algebraeon::sets::structure::MetaType;
use algebraeon::sets::structure::SetSignature;
use num_bigint::BigInt;
use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::exceptions::PyZeroDivisionError;
use pyo3::{IntoPyObjectExt, exceptions::PyTypeError, prelude::*};

#[pyclass(name = "Rat")]
#[derive(Clone)]
pub struct PythonRational {
    inner: Rational,
}

impl<'py> PythonCast<'py> for PythonRational {
    fn cast_equiv(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        let py = obj.py();
        if obj
            .get_type()
            .is(py.import("fractions")?.getattr("Fraction")?)
        {
            Ok(Self {
                inner: Rational::from_integers(
                    PythonInteger::py_new(&obj.getattr("numerator").unwrap())
                        .unwrap()
                        .inner(),
                    PythonInteger::py_new(&obj.getattr("denominator").unwrap())
                        .unwrap()
                        .inner(),
                ),
            })
        } else {
            Err(PyTypeError::new_err(format!(
                "Can't create a `Rat` from a `{}`",
                obj.get_type().repr()?
            )))
        }
    }

    fn cast_proper_subtype(obj: &Bound<'py, PyAny>) -> Option<Self> {
        if let Ok(n) = PythonInteger::cast_subtype(obj) {
            Some(Self {
                inner: Rational::from(n.inner()),
            })
        } else {
            None
        }
    }
}

impl PythonStructure for PythonRational {
    type Structure = RationalCanonicalStructure;

    fn structure(&self) -> Self::Structure {
        Rational::structure()
    }

    fn inner(&self) -> &<Self::Structure as SetSignature>::Set {
        &self.inner
    }
}

impl_pymethods_cmp!(PythonRational);
impl_pymethods_pos!(PythonRational);
impl_pymethods_add!(PythonRational);
impl_pymethods_neg!(PythonRational);
impl_pymethods_sub!(PythonRational);
impl_pymethods_mul!(PythonRational);
impl_pymethods_div!(PythonRational);
impl_pymethods_nat_pow!(PythonRational);

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
            if let Ok(obj1) = PythonInteger::py_new(obj1)
                && let Ok(obj2) = PythonInteger::py_new(obj2)
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
            Self::cast_subtype(obj1)
        }
    }

    pub fn __int__(&self) -> PyResult<BigInt> {
        if let Ok(n) = Integer::try_from(&self.inner) {
            Ok(algebraeon_to_bignum_int(&n))
        } else {
            Err(PyValueError::new_err(""))
        }
    }

    pub fn __str__(&self) -> String {
        format!("{}", self.inner)
    }

    pub fn __repr__(&self) -> String {
        format!("Rat({})", self.inner)
    }
}
