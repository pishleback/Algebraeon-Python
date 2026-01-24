use crate::PythonElement;
use crate::PythonElementCast;
use crate::PythonSet;
use crate::algebraeon_to_bignum_int;
use crate::bignum_to_algebraeon_int;
use crate::natural::PythonNaturalSet;
use algebraeon::nzq::Integer;
use algebraeon::nzq::IntegerCanonicalStructure;
use algebraeon::sets::structure::MetaType;
use algebraeon::sets::structure::SetSignature;
use num_bigint::BigInt;
use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::{IntoPyObjectExt, exceptions::PyTypeError, prelude::*};

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PythonIntegerSet {}

impl PythonSet for PythonIntegerSet {
    type Elem = PythonInteger;

    fn from_elem(&self, elem: Integer) -> Self::Elem {
        PythonInteger { inner: elem }
    }

    fn str(&self) -> String {
        "ℤ".to_string()
    }

    fn repr(&self) -> String {
        "Int".to_string()
    }
}

impl_pymethods_set!(PythonIntegerSet);

#[pyclass]
#[derive(Debug, Clone)]
pub struct PythonInteger {
    pub inner: Integer,
}

impl PythonElement for PythonInteger {
    type Set = PythonIntegerSet;

    type Structure = IntegerCanonicalStructure;

    fn structure(&self) -> Self::Structure {
        Integer::structure()
    }

    fn to_elem(&self) -> &<Self::Structure as SetSignature>::Set {
        &self.inner
    }

    fn into_elem(self) -> <Self::Structure as SetSignature>::Set {
        self.inner
    }

    fn set(&self) -> Self::Set {
        PythonIntegerSet {}
    }

    fn str(&self) -> String {
        format!("{}", self.inner)
    }

    fn repr(&self) -> String {
        format!("Int({})", self.inner)
    }
}

impl<'py> PythonElementCast<'py> for PythonIntegerSet {
    fn cast_exact(&self, obj: &Bound<'py, PyAny>) -> Option<Self::Elem> {
        obj.extract::<Self::Elem>().ok()
    }

    fn cast_equiv(&self, obj: &Bound<'py, PyAny>) -> PyResult<PythonInteger> {
        if let Ok(n) = obj.extract::<BigInt>() {
            Ok(PythonInteger {
                inner: bignum_to_algebraeon_int(&n),
            })
        } else {
            Err(PyTypeError::new_err(format!(
                "Can't create an `Int` from a `{}`",
                obj.get_type().repr()?
            )))
        }
    }

    fn cast_proper_subtype(&self, obj: &Bound<'py, PyAny>) -> Option<PythonInteger> {
        if let Ok(n) = PythonNaturalSet::default().cast_subtype(obj) {
            Some(PythonInteger {
                inner: Integer::from(n.to_elem()),
            })
        } else {
            None
        }
    }
}

impl_pymethods_elem!(PythonInteger);
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
        PythonIntegerSet::default().cast_subtype(obj)
    }

    pub fn __int__(&self) -> BigInt {
        algebraeon_to_bignum_int(&self.inner)
    }
}
