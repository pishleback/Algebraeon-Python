use ::algebraeon::{
    nzq::{Integer, Natural},
    sets::structure::SetSignature,
};
use num_bigint::{BigInt, BigUint};
use pyo3::{PyTypeInfo, prelude::*};

#[pymodule]
fn algebraeon(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<natural::PythonNatural>()?;
    m.add_class::<integer::PythonInteger>()?;
    m.add_class::<rational::PythonRational>()?;
    m.add_class::<natural_factored::PythonNaturalFactored>()?;

    m.add_function(wrap_pyfunction!(algebraeon_rust_library_version, m)?)?;
    m.add_function(wrap_pyfunction!(algebraeon_python_library_version, m)?)?;

    Ok(())
}

#[pyfunction]
fn algebraeon_python_library_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[pyfunction]
pub fn algebraeon_rust_library_version() -> &'static str {
    include_str!(concat!(env!("OUT_DIR"), "/algebraeon_dep_version.rs"))
}

#[allow(unused)]
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
        obj.extract::<Self>().ok()
    }

    fn cast_equiv(obj: &Bound<'py, PyAny>) -> PyResult<Self>;

    fn cast_proper_subtype(obj: &Bound<'py, PyAny>) -> Option<Self>;

    fn cast_subtype(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Some(obj) = Self::cast_exact(obj) {
            Ok(obj)
        } else if let Some(obj) = Self::cast_proper_subtype(obj) {
            Ok(obj)
        } else {
            Self::cast_equiv(obj)
        }
    }
}

trait PythonStructure {
    type Structure: SetSignature;

    fn structure(&self) -> Self::Structure;

    fn inner(&self) -> &<Self::Structure as SetSignature>::Set;
}

mod impl_macros;

pub mod integer;
pub mod natural;
pub mod natural_factored;
pub mod rational;
