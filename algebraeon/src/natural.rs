use crate::PythonElement;
use crate::PythonElementCast;
use crate::PythonSet;
use crate::algebraeon_to_bignum_nat;
use crate::bignum_to_algebraeon_int;
use ::algebraeon::nzq::Natural;
use ::algebraeon::nzq::NaturalCanonicalStructure;
use algebraeon::sets::structure::MetaType;
use algebraeon::sets::structure::SetSignature;
use num_bigint::{BigInt, BigUint};
use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::{IntoPyObjectExt, exceptions::PyTypeError, prelude::*};

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PythonNaturalSet {}

impl PythonSet for PythonNaturalSet {
    type Elem = PythonNatural;

    fn from_elem(&self, elem: Natural) -> Self::Elem {
        PythonNatural { inner: elem }
    }

    fn str(&self) -> String {
        "ℕ".to_string()
    }

    fn repr(&self) -> String {
        "Nat".to_string()
    }
}

impl_pymethods_set!(PythonNaturalSet);

#[pyclass]
#[derive(Debug, Clone)]
pub struct PythonNatural {
    pub inner: Natural,
}

impl PythonElement for PythonNatural {
    type Set = PythonNaturalSet;

    type Structure = NaturalCanonicalStructure;

    fn structure(&self) -> Self::Structure {
        Natural::structure()
    }

    fn to_elem(&self) -> &<Self::Structure as SetSignature>::Set {
        &self.inner
    }

    fn into_elem(self) -> <Self::Structure as SetSignature>::Set {
        self.inner
    }

    fn set(&self) -> Self::Set {
        PythonNaturalSet {}
    }

    fn str(&self) -> String {
        format!("{}", self.inner)
    }

    fn repr(&self) -> String {
        format!("Nat({})", self.inner)
    }
}

impl<'py> PythonElementCast<'py> for PythonNaturalSet {
    fn cast_exact(&self, obj: &Bound<'py, PyAny>) -> Option<Self::Elem> {
        obj.extract::<Self::Elem>().ok()
    }

    fn cast_equiv(&self, obj: &Bound<'py, PyAny>) -> PyResult<PythonNatural> {
        if let Ok(n) = obj.extract::<BigInt>() {
            if let Ok(n) = Natural::try_from(bignum_to_algebraeon_int(&n)) {
                Ok(PythonNatural { inner: n })
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

    fn cast_proper_subtype(&self, _obj: &Bound<'py, PyAny>) -> Option<PythonNatural> {
        None
    }
}

impl_pymethods_elem!(PythonNatural);
impl_pymethods_eq!(PythonNatural);
impl_pymethods_pos!(PythonNatural);
impl_pymethods_add!(PythonNatural);
impl_pymethods_try_neg!(PythonNatural);
impl_pymethods_try_sub!(PythonNatural);
impl_pymethods_mul!(PythonNatural);
impl_pymethods_nat_pow!(PythonNatural);

#[pymethods]
impl PythonNatural {
    #[new]
    pub fn py_new<'py>(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        PythonNaturalSet::default().cast_subtype(obj)
    }

    pub fn __int__(&self) -> BigUint {
        algebraeon_to_bignum_nat(&self.inner)
    }
}
