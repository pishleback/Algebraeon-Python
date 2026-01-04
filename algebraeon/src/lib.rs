use ::algebraeon::rings::structure::AdditiveGroupSignature;
use ::algebraeon::rings::structure::SemiRingSignature;
use ::algebraeon::sets::structure::EqSignature;
use ::algebraeon::{
    nzq::{
        Integer, IntegerCanonicalStructure, Natural, NaturalCanonicalStructure, Rational,
        RationalCanonicalStructure,
    },
    rings::structure::AdditiveMonoidSignature,
    sets::structure::{MetaType, SetSignature, Signature},
};
use num_bigint::{BigInt, BigUint};
use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::{IntoPyObjectExt, PyTypeInfo, exceptions::PyTypeError, prelude::*};

#[pymodule]
fn algebraeon(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PythonNatural>()?;
    m.add_class::<PythonInteger>()?;
    m.add_class::<PythonRational>()?;

    // m.add_class::<natural::PythonNatural>()?;

    // m.add_function(wrap_pyfunction!(foo, m)?)?;

    Ok(())
}

fn bignum_to_algebraeon_nat(x: &BigUint) -> Natural {
    // TODO: use a more efficient method
    use std::str::FromStr;
    Natural::from_str(x.to_string().as_str()).unwrap()
}

fn algebraeon_to_bignum_nat(x: &Natural) -> BigUint {
    // TODO: use a more efficient method
    use std::str::FromStr;
    BigUint::from_str(x.to_string().as_str()).unwrap()
}

fn bignum_to_algebraeon_int(x: &BigInt) -> Integer {
    // TODO: use a more efficient method
    use std::str::FromStr;
    Integer::from_str(x.to_string().as_str()).unwrap()
}

fn algebraeon_to_bignum_int(x: &Integer) -> BigInt {
    // TODO: use a more efficient method
    use std::str::FromStr;
    BigInt::from_str(x.to_string().as_str()).unwrap()
}

trait PythonCast<'py>: Sized + for<'a> FromPyObject<'a, 'py> + PyTypeInfo {
    fn cast_exact(obj: &Bound<'py, PyAny>) -> Option<Self> {
        if let Ok(obj) = obj.extract::<Self>() {
            Some(obj)
        } else {
            None
        }
    }

    fn cast_equiv(obj: &Bound<'py, PyAny>) -> Option<Option<Self>>;

    fn cast_proper_subtype(obj: &Bound<'py, PyAny>) -> Option<Self>;

    fn cast_subtype(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        let orig_obj = obj;
        let py = obj.py();
        if let Some(obj) = Self::cast_exact(obj) {
            Ok(obj)
        } else if let Some(obj) = Self::cast_equiv(obj) {
            if let Some(obj) = obj {
                Ok(obj)
            } else {
                Err(PyValueError::new_err(format!(
                    "Can't cast `{}` to `{}`",
                    orig_obj.repr()?,
                    Self::type_object(py).name()?
                )))
            }
        } else if let Some(obj) = Self::cast_proper_subtype(obj) {
            Ok(obj)
        } else {
            Err(PyTypeError::new_err(format!(
                "Can't cast `{}` to `{}`",
                orig_obj.repr()?,
                Self::type_object(py).name()?
            )))
        }
    }
}

trait PythonStructure {
    type Structure: SetSignature;

    fn structure(&self) -> Self::Structure;

    fn inner(&self) -> &<Self::Structure as SetSignature>::Set;
}

macro_rules! impl_pymethods_eq {
    ($python_type:ident) => {
        #[pymethods]
        impl $python_type {
            fn __richcmp__<'py>(
                &self,
                other: &Bound<'py, PyAny>,
                op: CompareOp,
            ) -> PyResult<Py<PyAny>> {
                let py = other.py();
                if let Ok(other) = Self::py_new(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    let eq_result = structure.equal(self.inner(), other.inner());
                    match op {
                        CompareOp::Eq => Ok(eq_result.into_py_any(py)?),
                        CompareOp::Ne => Ok((!eq_result).into_py_any(py)?),
                        CompareOp::Lt | CompareOp::Le | CompareOp::Gt | CompareOp::Ge => {
                            Ok(py.NotImplemented())
                        }
                    }
                } else {
                    Ok(py.NotImplemented())
                }
            }
        }
    };
}

macro_rules! impl_pymethods_add {
    ($python_type:ident) => {
        #[pymethods]
        impl $python_type {
            fn __add__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
                let py = other.py();
                if let Ok(other) = Self::py_new(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    Ok(Self {
                        inner: structure.add(self.inner(), other.inner()),
                    }
                    .into_py_any(py)?)
                } else {
                    Ok(py.NotImplemented())
                }
            }

            fn __radd__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
                let py = other.py();
                if let Ok(other) = Self::py_new(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    Ok(Self {
                        inner: structure.add(other.inner(), self.inner()),
                    }
                    .into_py_any(py)?)
                } else {
                    Ok(py.NotImplemented())
                }
            }
        }
    };
}

macro_rules! impl_pymethods_pos {
    ($python_type:ident) => {
        #[pymethods]
        impl $python_type {
            fn __pos__<'py>(&self, py: Python<'py>) -> PyResult<Py<PyAny>> {
                Ok(Self {
                    inner: self.inner().clone(),
                }
                .into_py_any(py)?)
            }
        }
    };
}

macro_rules! impl_pymethods_neg {
    ($python_type:ident) => {
        #[pymethods]
        impl $python_type {
            fn __neg__<'py>(&self, py: Python<'py>) -> PyResult<Py<PyAny>> {
                Ok(Self {
                    inner: self.structure().neg(self.inner()),
                }
                .into_py_any(py)?)
            }
        }
    };
}

macro_rules! impl_pymethods_sub {
    ($python_type:ident) => {
        #[pymethods]
        impl $python_type {
            fn __sub__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
                let py = other.py();
                if let Ok(other) = Self::py_new(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    Ok(Self {
                        inner: structure.sub(self.inner(), other.inner()),
                    }
                    .into_py_any(py)?)
                } else {
                    Ok(py.NotImplemented())
                }
            }

            fn __rsub__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
                let py = other.py();
                if let Ok(other) = Self::py_new(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    Ok(Self {
                        inner: structure.sub(other.inner(), self.inner()),
                    }
                    .into_py_any(py)?)
                } else {
                    Ok(py.NotImplemented())
                }
            }
        }
    };
}

macro_rules! impl_pymethods_mul {
    ($python_type:ident) => {
        #[pymethods]
        impl $python_type {
            fn __mul__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
                let py = other.py();
                if let Ok(other) = Self::py_new(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    Ok(Self {
                        inner: structure.mul(self.inner(), other.inner()),
                    }
                    .into_py_any(py)?)
                } else {
                    Ok(py.NotImplemented())
                }
            }

            fn __rmul__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
                let py = other.py();
                if let Ok(other) = Self::py_new(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    Ok(Self {
                        inner: structure.mul(other.inner(), self.inner()),
                    }
                    .into_py_any(py)?)
                } else {
                    Ok(py.NotImplemented())
                }
            }
        }
    };
}

macro_rules! impl_pymethods_nat_pow {
    ($python_type:ident) => {
        #[pymethods]
        impl $python_type {
            fn __pow__<'py>(
                &self,
                other: &Bound<'py, PyAny>,
                modulus: &Bound<'py, PyAny>,
            ) -> PyResult<Py<PyAny>> {
                let py = other.py();
                if !modulus.is_none() {
                    Ok(py.NotImplemented())
                } else {
                    if let Ok(other) = PythonNatural::py_new(other) {
                        Ok(Self {
                            inner: self.structure().nat_pow(self.inner(), other.inner()),
                        }
                        .into_py_any(py)?)
                    } else {
                        Ok(py.NotImplemented())
                    }
                }
            }

            fn __rpow__<'py>(
                &self,
                other: &Bound<'py, PyAny>,
                _modulus: &Bound<'py, PyAny>,
            ) -> PyResult<Py<PyAny>> {
                let py = other.py();
                Ok(py.NotImplemented())
            }
        }
    };
}

#[pyclass(name = "Nat")]
#[derive(Clone)]
pub struct PythonNatural {
    inner: Natural,
}

impl<'py> PythonCast<'py> for PythonNatural {
    fn cast_equiv(obj: &Bound<'py, PyAny>) -> Option<Option<Self>> {
        if let Ok(n) = obj.extract::<BigInt>() {
            Some(
                if let Ok(n) = Natural::try_from(bignum_to_algebraeon_int(&n)) {
                    Some(Self { inner: n })
                } else {
                    None
                },
            )
        } else {
            None
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
    fn py_new<'py>(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Self::cast_subtype(obj)
    }

    fn __int__(&self) -> BigUint {
        algebraeon_to_bignum_nat(&self.inner)
    }

    fn __str__(&self) -> String {
        format!("{}", self.inner)
    }

    fn __repr__(&self) -> String {
        format!("Nat({})", self.inner)
    }
}

#[pyclass(name = "Int")]
#[derive(Clone)]
pub struct PythonInteger {
    inner: Integer,
}

impl<'py> PythonCast<'py> for PythonInteger {
    fn cast_equiv(obj: &Bound<'py, PyAny>) -> Option<Option<Self>> {
        if let Ok(n) = obj.extract::<BigInt>() {
            Some(Some(Self {
                inner: bignum_to_algebraeon_int(&n),
            }))
        } else {
            None
        }
    }

    fn cast_proper_subtype(obj: &Bound<'py, PyAny>) -> Option<Self> {
        if let Ok(n) = PythonNatural::cast_subtype(obj) {
            Some(Self {
                inner: Integer::from(n.inner),
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

impl_pymethods_eq!(PythonInteger);
impl_pymethods_pos!(PythonInteger);
impl_pymethods_add!(PythonInteger);
impl_pymethods_neg!(PythonInteger);
impl_pymethods_sub!(PythonInteger);
impl_pymethods_mul!(PythonInteger);
impl_pymethods_nat_pow!(PythonInteger);

#[pymethods]
impl PythonInteger {
    #[new]
    fn py_new<'py>(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Self::cast_subtype(obj)
    }

    fn __int__(&self) -> BigInt {
        algebraeon_to_bignum_int(&self.inner)
    }

    fn __str__(&self) -> String {
        format!("{}", self.inner)
    }

    fn __repr__(&self) -> String {
        format!("Int({})", self.inner)
    }
}

#[pyclass(name = "Rat")]
#[derive(Clone)]
pub struct PythonRational {
    inner: Rational,
}

impl<'py> PythonCast<'py> for PythonRational {
    fn cast_equiv(_obj: &Bound<'py, PyAny>) -> Option<Option<Self>> {
        None
    }

    fn cast_proper_subtype(obj: &Bound<'py, PyAny>) -> Option<Self> {
        if let Ok(n) = PythonInteger::cast_subtype(obj) {
            Some(Self {
                inner: Rational::from(n.inner),
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

impl_pymethods_eq!(PythonRational);
impl_pymethods_pos!(PythonRational);
impl_pymethods_add!(PythonRational);
impl_pymethods_neg!(PythonRational);
impl_pymethods_sub!(PythonRational);
impl_pymethods_mul!(PythonRational);
impl_pymethods_nat_pow!(PythonRational);

#[pymethods]
impl PythonRational {
    #[new]
    fn py_new<'py>(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Self::cast_subtype(obj)
    }

    fn __int__(&self) -> PyResult<BigInt> {
        if let Ok(n) = Integer::try_from(&self.inner) {
            Ok(algebraeon_to_bignum_int(&n))
        } else {
            Err(PyValueError::new_err(""))
        }
    }

    fn __str__(&self) -> String {
        format!("{}", self.inner)
    }

    fn __repr__(&self) -> String {
        format!("Rat({})", self.inner)
    }
}
