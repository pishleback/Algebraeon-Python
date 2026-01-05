use crate::PythonCast;
use crate::PythonStructure;
use crate::algebraeon_to_bignum_int;
use crate::bignum_to_algebraeon_int;
use crate::impl_pymethods_add;
use crate::impl_pymethods_cmp;
use crate::impl_pymethods_div;
use crate::impl_pymethods_mul;
use crate::impl_pymethods_nat_pow;
use crate::impl_pymethods_neg;
use crate::impl_pymethods_pos;
use crate::impl_pymethods_sub;
use crate::natural::PythonNatural;
use algebraeon::nzq::Integer;
use algebraeon::nzq::IntegerCanonicalStructure;
use algebraeon::sets::structure::MetaType;
use algebraeon::sets::structure::SetSignature;
use num_bigint::BigInt;
use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::exceptions::PyZeroDivisionError;
use pyo3::{IntoPyObjectExt, exceptions::PyTypeError, prelude::*};

#[pyclass(name = "Int")]
#[derive(Clone)]
pub struct PythonInteger {
    inner: Integer,
}

impl<'py> PythonCast<'py> for PythonInteger {
    fn cast_equiv(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(n) = obj.extract::<BigInt>() {
            Ok(Self {
                inner: bignum_to_algebraeon_int(&n),
            })
        } else {
            Err(PyTypeError::new_err(format!(
                "Can't create an `Int` from a `{}`",
                obj.get_type().repr()?
            )))
        }
    }

    fn cast_proper_subtype(obj: &Bound<'py, PyAny>) -> Option<Self> {
        if let Ok(n) = PythonNatural::cast_subtype(obj) {
            Some(Self {
                inner: Integer::from(n.inner()),
            })
        } else {
            None
        }
    }
}

impl PythonStructure for PythonInteger {
    type Structure = IntegerCanonicalStructure;

    fn structure(&self) -> Self::Structure {
        Integer::structure()
    }

    fn inner(&self) -> &<Self::Structure as SetSignature>::Set {
        &self.inner
    }
}

impl_pymethods_cmp!(PythonInteger);
impl_pymethods_pos!(PythonInteger);
impl_pymethods_add!(PythonInteger);
impl_pymethods_neg!(PythonInteger);
impl_pymethods_sub!(PythonInteger);
impl_pymethods_mul!(PythonInteger);
impl_pymethods_div!(PythonInteger);
impl_pymethods_nat_pow!(PythonInteger);

#[pymethods]
impl PythonInteger {
    #[new]
    pub fn py_new<'py>(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Self::cast_subtype(obj)
    }

    pub fn __int__(&self) -> BigInt {
        algebraeon_to_bignum_int(&self.inner)
    }

    pub fn __str__(&self) -> String {
        format!("{}", self.inner)
    }

    pub fn __repr__(&self) -> String {
        format!("Int({})", self.inner)
    }
}
