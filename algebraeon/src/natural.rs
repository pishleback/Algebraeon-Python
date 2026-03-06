use crate::CastError;
use crate::PythonElement;
use crate::PythonElementCast;
use crate::PythonSet;
use crate::algebraeon_to_bignum_nat;
use crate::bignum_to_algebraeon_int;
use crate::integer::PythonIntegerSet;
use ::algebraeon::nzq::Natural;
use ::algebraeon::nzq::NaturalCanonicalStructure;
use algebraeon::nzq::primes;
use algebraeon::sets::structure::MetaType;
use algebraeon::sets::structure::SetSignature;
use num_bigint::{BigInt, BigUint};
use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::{IntoPyObjectExt, prelude::*};

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

py_iterator_for!(PythonNatural, PythonNaturalPrimeIterator);

#[pymethods]
impl PythonNaturalSet {
    pub fn primes(&self) -> PythonNaturalPrimeIterator {
        PythonNaturalPrimeIterator::new(Box::new(primes().map(|p| PythonNatural {
            inner: Natural::from(p),
        })))
    }
}

#[pyclass(name = "Nat")]
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
    fn proper_subset_cast_impl(&self, obj: &Bound<'py, PyAny>) -> Result<Self::Elem, CastError> {
        if let Ok(n) = obj.extract::<BigInt>() {
            if let Ok(n) = Natural::try_from(bignum_to_algebraeon_int(&n)) {
                return Ok(PythonNatural { inner: n });
            } else {
                return Err(CastError::Value);
            }
        }
        Err(CastError::Type)
    }

    fn proper_supset_cast_impl(&self, obj: &Bound<'py, PyAny>) -> Result<Self::Elem, CastError> {
        match PythonIntegerSet::default().supset_cast_impl(obj) {
            Ok(obj) => {
                if let Ok(obj) = Natural::try_from(obj.inner) {
                    return Ok(PythonNatural { inner: obj });
                } else {
                    return Err(CastError::Value);
                }
            }
            Err(CastError::Value) => {
                return Err(CastError::Value);
            }
            Err(CastError::Type) => {}
        }
        Err(CastError::Type)
    }

    fn other_implicit_cast_impl(&self, _obj: &Bound<'py, PyAny>) -> Result<Self::Elem, CastError> {
        Err(CastError::Type)
    }

    fn other_explicit_cast_impl(&self, _obj: &Bound<'py, PyAny>) -> Result<Self::Elem, CastError> {
        Err(CastError::Type)
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
        PythonNaturalSet::default().explicit_cast(obj)
    }

    pub fn __int__(&self) -> BigUint {
        algebraeon_to_bignum_nat(&self.inner)
    }
}
