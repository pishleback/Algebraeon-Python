use crate::PythonCast;
use crate::PythonStructure;
use crate::algebraeon_to_bignum_nat;
use crate::bignum_to_algebraeon_int;
use crate::impl_pymethods_add;
use crate::impl_pymethods_eq;
use crate::impl_pymethods_mul;
use crate::impl_pymethods_nat_pow;
use crate::impl_pymethods_pos;
use crate::natural_factored::PythonNaturalFactored;
use ::algebraeon::nzq::Natural;
use ::algebraeon::nzq::NaturalCanonicalStructure;
use algebraeon::sets::structure::MetaType;
use algebraeon::sets::structure::SetSignature;
use num_bigint::{BigInt, BigUint};
use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::{IntoPyObjectExt, exceptions::PyTypeError, prelude::*};

#[pyclass(name = "Nat")]
#[derive(Clone)]
pub struct PythonNatural {
    inner: Natural,
}

impl<'py> PythonCast<'py> for PythonNatural {
    fn cast_equiv(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(n) = obj.extract::<BigInt>() {
            if let Ok(n) = Natural::try_from(bignum_to_algebraeon_int(&n)) {
                Ok(Self { inner: n })
            } else {
                Err(PyValueError::new_err(format!(
                    "Can't create a `Nat` from `{}`",
                    obj.repr()?
                )))
            }
        } else {
            Err(PyTypeError::new_err(format!(
                "Can't create a `Nat` from a `{}`",
                obj.get_type().repr()?
            )))
        }
    }

    fn cast_proper_subtype(_obj: &Bound<'py, PyAny>) -> Option<Self> {
        None
    }
}

impl PythonStructure for PythonNatural {
    type Structure = NaturalCanonicalStructure;

    fn structure(&self) -> Self::Structure {
        Natural::structure()
    }

    fn inner(&self) -> &<Self::Structure as SetSignature>::Set {
        &self.inner
    }
}

impl_pymethods_eq!(PythonNatural);
impl_pymethods_pos!(PythonNatural);
impl_pymethods_add!(PythonNatural);
impl_pymethods_mul!(PythonNatural);
impl_pymethods_nat_pow!(PythonNatural);

#[pymethods]
impl PythonNatural {
    #[new]
    pub fn py_new<'py>(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Self::cast_subtype(obj)
    }

    pub fn __int__(&self) -> BigUint {
        algebraeon_to_bignum_nat(&self.inner)
    }

    pub fn __str__(&self) -> String {
        format!("{}", self.inner)
    }

    pub fn __repr__(&self) -> String {
        format!("Nat({})", self.inner)
    }

    pub fn factor(&self) -> PythonNaturalFactored {
        PythonNaturalFactored::from_nat(&self.inner)
    }

    pub fn is_prime(&self) -> bool {
        self.factor().is_prime()
    }
}
