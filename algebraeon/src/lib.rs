use std::f32::consts::E;

use ::algebraeon::nzq::{Integer, Natural};
use num_bigint::{BigInt, BigUint};
use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
};

#[pymodule]
fn algebraeon(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PythonSet>()?;

    m.add_class::<PythonNatural>()?;

    // m.add_class::<natural::PythonNatural>()?;

    // m.add_function(wrap_pyfunction!(foo, m)?)?;

    Ok(())
}

pub fn bignum_to_algebraeon_nat(x: &BigUint) -> Natural {
    // TODO: use a more efficient method
    use std::str::FromStr;
    Natural::from_str(x.to_string().as_str()).unwrap()
}

pub fn algebraeon_to_bignum_nat(x: &Natural) -> BigUint {
    // TODO: use a more efficient method
    use std::str::FromStr;
    BigUint::from_str(x.to_string().as_str()).unwrap()
}

pub fn bignum_to_algebraeon_int(x: &BigInt) -> Integer {
    // TODO: use a more efficient method
    use std::str::FromStr;
    Integer::from_str(x.to_string().as_str()).unwrap()
}

pub fn algebraeon_to_bignum_int(x: &Integer) -> BigInt {
    // TODO: use a more efficient method
    use std::str::FromStr;
    BigInt::from_str(x.to_string().as_str()).unwrap()
}

#[derive(Clone)]
pub enum Set {
    Naturals,
    Integers,
    Rationals,
    Modulo(Natural),
    Polynomials(Box<Set>),
    Matricies(Box<Set>),
}

impl Set {
    fn __repr__(&self) -> String {
        match self {
            Set::Naturals => "Naturals".to_string(),
            Set::Integers => "Integers".to_string(),
            Set::Rationals => "Rationals".to_string(),
            Set::Modulo(n) => format!("Modulo({n})"),
            Set::Polynomials(coeff_set) => format!("Polynomial({})", coeff_set.__repr__()),
            Set::Matricies(entry_set) => format!("Matricies({})", entry_set.__repr__()),
        }
    }
}

#[pyclass(name = "Set", module = "algebraeon")]
#[derive(Clone)]
pub struct PythonSet {
    inner: Set,
}

#[pymethods]
impl PythonSet {
    #[staticmethod]
    fn naturals() -> Self {
        Self {
            inner: Set::Naturals,
        }
    }

    #[staticmethod]
    fn integers() -> Self {
        Self {
            inner: Set::Integers,
        }
    }

    #[staticmethod]
    fn rationals() -> Self {
        Self {
            inner: Set::Rationals,
        }
    }

    #[staticmethod]
    fn modulo<'py>(n: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            inner: Set::Modulo(PythonNatural::py_new(n)?.inner),
        })
    }

    #[staticmethod]
    fn polynomials<'py>(set: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(set) = set.extract::<Self>() {
            Ok(Self {
                inner: Set::Polynomials(Box::new(set.inner)),
            })
        } else {
            Err(PyTypeError::new_err(format!("`{:?}` is not a `Set`", set)))
        }
    }

    #[staticmethod]
    fn matricies<'py>(set: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(set) = set.extract::<Self>() {
            Ok(Self {
                inner: Set::Matricies(Box::new(set.inner)),
            })
        } else {
            Err(PyTypeError::new_err(format!("`{:?}` is not a `Set`", set)))
        }
    }

    fn __repr__(&self) -> String {
        self.inner.__repr__()
    }
}

pub enum Element {
    Natural,
    Integer,
}

fn cast_element<'py>(element: &Bound<'py, PyAny>) -> (PythonSet, Element) {}

#[pyclass(name = "Nat")]
#[derive(Clone)]
pub struct PythonNatural {
    inner: Natural,
}

#[pymethods]
impl PythonNatural {
    #[new]
    fn py_new<'py>(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(n) = obj.extract::<Self>() {
            Ok(n)
        } else if let Ok(n) = obj.extract::<BigUint>() {
            Ok(Self {
                inner: bignum_to_algebraeon_nat(&n),
            })
        } else {
            Err(PyValueError::new_err(format!(
                "Can't create an instance of `Nat` from `{:?}`",
                obj
            )))
        }
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
