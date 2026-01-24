use ::algebraeon::{
    nzq::{Integer, Natural},
    sets::structure::SetSignature,
};
use num_bigint::{BigInt, BigUint};

use pyo3::{PyTypeInfo, prelude::*};

#[pymodule]
fn algebraeon(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add(
        "Nat",
        Py::new(m.py(), natural::PythonNaturalSet::default())?,
    )?;
    m.add(
        "Int",
        Py::new(m.py(), integer::PythonIntegerSet::default())?,
    )?;
    m.add(
        "Rat",
        Py::new(m.py(), rational::PythonRationalSet::default())?,
    )?;

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

pub trait PythonSet: PartialEq + Eq {
    type Elem: PythonElement<Set = Self> + PyTypeInfo;

    #[allow(clippy::wrong_self_convention)]
    fn from_elem(
        &self,
        elem: <<Self::Elem as PythonElement>::Structure as SetSignature>::Set,
    ) -> Self::Elem;
    fn str(&self) -> String;
    fn repr(&self) -> String;
}

pub trait PythonElementCast<'py>: PythonSet
where
    Self::Elem: Sized + for<'a> FromPyObject<'a, 'py> + PyTypeInfo,
{
    fn cast_exact(&self, obj: &Bound<'py, PyAny>) -> Option<Self::Elem>;

    fn cast_equiv(&self, obj: &Bound<'py, PyAny>) -> PyResult<Self::Elem>;

    fn cast_proper_subtype(&self, obj: &Bound<'py, PyAny>) -> Option<Self::Elem>;

    fn cast_subtype(&self, obj: &Bound<'py, PyAny>) -> PyResult<Self::Elem> {
        if let Some(obj) = self.cast_exact(obj) {
            Ok(obj)
        } else if let Some(obj) = self.cast_proper_subtype(obj) {
            Ok(obj)
        } else {
            self.cast_equiv(obj)
        }
    }
}

#[macro_export]
macro_rules! impl_pymethods_set {
    ($python_type:ident) => {
        static_assertions::const_assert!(impls::impls!($python_type : $crate::PythonSet));

        #[pymethods]
        impl $python_type {
            pub fn __richcmp__<'py>(
                &self,
                other: &Bound<'py, PyAny>,
                op: CompareOp,
            ) -> PyResult<Py<PyAny>> {
                let py = other.py();
                if let Ok(other) = other.extract::<Self>() {
                    match op {
                        CompareOp::Eq => Ok((*self == other).into_py_any(py)?),
                        CompareOp::Ne => Ok((*self != other).into_py_any(py)?),
                        CompareOp::Lt | CompareOp::Le | CompareOp::Gt | CompareOp::Ge => {
                            Ok(py.NotImplemented())
                        }
                    }
                } else {
                    Ok(py.NotImplemented())
                }
            }

            #[pyo3(signature = (*args, **kwargs))]
            pub fn __call__(
                &mut self,
                args: &Bound<'_, pyo3::types::PyTuple>,
                kwargs: Option<&Bound<'_, pyo3::types::PyDict>>,
            ) -> PyResult<Py<PyAny>> {
                use pyo3::PyTypeInfo;
                let py = args.py();
                if args.len() == 1 && kwargs.is_none() {
                    let first = args.get_item(0)?;
                    if let Ok(result) = self.cast_subtype(&first) {
                        return result.into_py_any(py);
                    }
                }
                <Self as $crate::PythonSet>::Elem::type_object(py)
                    .call(args, kwargs)?
                    .into_py_any(py)
            }

            pub fn __str__(&self) -> String {
                self.str()
            }

            pub fn __repr__(&self) -> String {
                self.repr()
            }
        }
    };
}

pub trait PythonPolynomialSet: PythonSet {
    fn var(&self) -> <Self as PythonSet>::Elem;
}

#[macro_export]
macro_rules! impl_pymethods_polynomial_set {
    ($python_type:ident) => {
        #[pymethods]
        impl $python_type {
            pub fn var(&self) -> <$python_type as PythonSet>::Elem {
                PythonPolynomialSet::var(self)
            }
        }
    };
}

pub trait PythonToPolynomialSet: PythonSet {
    type PolynomialSet: PythonPolynomialSet;

    fn polynomials(&self) -> Self::PolynomialSet;
}

#[macro_export]
macro_rules! impl_pymethods_to_polynomial_set {
    ($python_type:ident) => {
        #[pymethods]
        impl $python_type {
            pub fn polynomials(&self) -> <$python_type as PythonToPolynomialSet>::PolynomialSet {
                PythonToPolynomialSet::polynomials(self)
            }
        }
    };
}

pub trait PythonElement {
    type Structure: SetSignature;
    type Set: PythonSet<Elem = Self>;

    fn set(&self) -> Self::Set;
    fn str(&self) -> String;
    fn repr(&self) -> String;

    fn structure(&self) -> Self::Structure;
    fn to_elem(&self) -> &<Self::Structure as SetSignature>::Set;
    fn into_elem(self) -> <Self::Structure as SetSignature>::Set;
}

#[macro_export]
macro_rules! impl_pymethods_elem {
    ($python_type:ident) => {
        static_assertions::const_assert!(impls::impls!($python_type : $crate::PythonElement));

        #[pymethods]
        impl $python_type {
            pub fn set(&self) -> <Self as $crate::PythonElement>::Set {
                PythonElement::set(self)
            }

            pub fn __str__(&self) -> String {
                self.str()
            }

            pub fn __repr__(&self) -> String {
                self.repr()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_pymethods_eq {
    ($python_type:ident) => {
        #[pymethods]
        impl $python_type {
            fn __richcmp__<'py>(
                &self,
                other: &Bound<'py, PyAny>,
                op: CompareOp,
            ) -> PyResult<Py<PyAny>> {
                use ::algebraeon::sets::structure::EqSignature;
                static_assertions::const_assert!(impls::impls!($python_type : PythonElement));
                static_assertions::const_assert!(
                    impls::impls!(<$python_type as $crate::PythonElement>::Structure : EqSignature)
                );
                let py = other.py();
                if let Ok(other) = self.set().cast_subtype(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    let eq_result = structure.equal(self.to_elem(), other.to_elem());
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

#[macro_export]
macro_rules! impl_pymethods_cmp {
    ($python_type:ident) => {
        #[pymethods]
        impl $python_type {
            fn __richcmp__<'py>(
                &self,
                other: &Bound<'py, PyAny>,
                op: CompareOp,
            ) -> PyResult<Py<PyAny>> {
                use ::algebraeon::sets::structure::OrdSignature;
                static_assertions::const_assert!(impls::impls!($python_type : PythonElement));
                static_assertions::const_assert!(
                    impls::impls!(<$python_type as $crate::PythonElement>::Structure : OrdSignature)
                );
                let py = other.py();
                if let Ok(other) = self.set().cast_subtype(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    let cmp_result = structure.cmp(self.to_elem(), other.to_elem());
                    match op {
                        CompareOp::Eq => Ok(cmp_result.is_eq().into_py_any(py)?),
                        CompareOp::Ne => Ok(cmp_result.is_ne().into_py_any(py)?),
                        CompareOp::Lt => Ok(cmp_result.is_lt().into_py_any(py)?),
                        CompareOp::Le => Ok(cmp_result.is_le().into_py_any(py)?),
                        CompareOp::Gt => Ok(cmp_result.is_gt().into_py_any(py)?),
                        CompareOp::Ge => Ok(cmp_result.is_ge().into_py_any(py)?),
                    }
                } else {
                    Ok(py.NotImplemented())
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_pymethods_add {
    ($python_type:ident) => {
        #[pymethods]
        impl $python_type {
            fn __add__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
                use ::algebraeon::rings::structure::AdditionSignature;
                static_assertions::const_assert!(impls::impls!($python_type : PythonElement));
                static_assertions::const_assert!(
                    impls::impls!(<$python_type as $crate::PythonElement>::Structure : AdditionSignature)
                );
                let py = other.py();
                let set = self.set();
                if let Ok(other) = set.cast_subtype(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    Ok(set.from_elem(structure.add(self.to_elem(), other.to_elem())).into_py_any(py)?)
                } else {
                    Ok(py.NotImplemented())
                }
            }

            fn __radd__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
                use ::algebraeon::rings::structure::AdditionSignature;
                static_assertions::const_assert!(impls::impls!($python_type : PythonElement));
                static_assertions::const_assert!(
                    impls::impls!(<$python_type as $crate::PythonElement>::Structure : AdditionSignature)
                );
                let py = other.py();
                let set = self.set();
                if let Ok(other) = set.cast_subtype(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    Ok(set.from_elem(structure.add(other.to_elem(), self.to_elem())).into_py_any(py)?)
                } else {
                    Ok(py.NotImplemented())
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_pymethods_pos {
    ($python_type:ident) => {
        #[pymethods]
        impl $python_type {
            fn __pos__<'py>(&self, py: Python<'py>) -> PyResult<Py<PyAny>> {
                use ::algebraeon::rings::structure::AdditiveMonoidSignature;
                static_assertions::const_assert!(impls::impls!($python_type : PythonElement));
                static_assertions::const_assert!(
                    impls::impls!(<$python_type as $crate::PythonElement>::Structure : AdditiveMonoidSignature)
                );
                self.clone().into_py_any(py)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_pymethods_neg {
    ($python_type:ident) => {
        #[pymethods]
        impl $python_type {
            fn __neg__<'py>(&self, py: Python<'py>) -> PyResult<Py<PyAny>> {
                use ::algebraeon::rings::structure::AdditiveGroupSignature;
                static_assertions::const_assert!(impls::impls!($python_type : PythonElement));
                static_assertions::const_assert!(
                    impls::impls!(<$python_type as $crate::PythonElement>::Structure : AdditiveGroupSignature)
                );
                self.set().from_elem(self.structure().neg(self.to_elem())).into_py_any(py)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_pymethods_sub {
    ($python_type:ident) => {
        #[pymethods]
        impl $python_type {
            fn __sub__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
                use ::algebraeon::rings::structure::AdditiveGroupSignature;
                static_assertions::const_assert!(impls::impls!($python_type : PythonElement));
                static_assertions::const_assert!(
                    impls::impls!(<$python_type as $crate::PythonElement>::Structure : AdditiveGroupSignature)
                );
                let py = other.py();
                let set = self.set();
                if let Ok(other) = set.cast_subtype(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    Ok(set.from_elem(structure.sub(self.to_elem(), other.to_elem())).into_py_any(py)?)
                } else {
                    Ok(py.NotImplemented())
                }
            }

            fn __rsub__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
                use ::algebraeon::rings::structure::AdditiveGroupSignature;
                static_assertions::const_assert!(impls::impls!($python_type : PythonElement));
                static_assertions::const_assert!(
                    impls::impls!(<$python_type as $crate::PythonElement>::Structure : AdditiveGroupSignature)
                );
                let py = other.py();
                let set = self.set();
                if let Ok(other) = set.cast_subtype(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    Ok(set.from_elem(structure.sub(other.to_elem(), self.to_elem())).into_py_any(py)?)
                } else {
                    Ok(py.NotImplemented())
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_pymethods_try_neg {
    ($python_type:ident) => {
        #[pymethods]
        impl $python_type {
            fn __neg__<'py>(&self, py: Python<'py>) -> PyResult<Py<PyAny>> {
                use ::algebraeon::rings::structure::TryNegateSignature;
                static_assertions::const_assert!(impls::impls!($python_type : PythonElement));
                static_assertions::const_assert!(
                    impls::impls!(<$python_type as $crate::PythonElement>::Structure : TryNegateSignature)
                );
                if let Some(repr) = self.structure().try_neg(self.to_elem()) {
                    self.set().from_elem(repr).into_py_any(py)
                } else {
                    Err(PyValueError::new_err(""))
                }
            }
        }
    }
}

#[macro_export]
macro_rules! impl_pymethods_try_sub {
    ($python_type:ident) => {
        #[pymethods]
        impl $python_type {
            fn __sub__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
                use ::algebraeon::rings::structure::CancellativeAdditionSignature;
                static_assertions::const_assert!(impls::impls!($python_type : PythonElement));
                static_assertions::const_assert!(
                    impls::impls!(<$python_type as $crate::PythonElement>::Structure : CancellativeAdditionSignature)
                );
                let py = other.py();
                let set = self.set();
                if let Ok(other) = set.cast_subtype(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    if let Some(repr) = structure.try_sub(self.to_elem(), other.to_elem()) {
                        Ok(set.from_elem(repr).into_py_any(py)?)
                    } else {
                        Err(PyValueError::new_err(""))
                    }
                } else {
                    Ok(py.NotImplemented())
                }
            }

            fn __rsub__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
                use ::algebraeon::rings::structure::CancellativeAdditionSignature;
                static_assertions::const_assert!(impls::impls!($python_type : PythonElement));
                static_assertions::const_assert!(
                    impls::impls!(<$python_type as $crate::PythonElement>::Structure : CancellativeAdditionSignature)
                );
                let py = other.py();
                let set = self.set();
                if let Ok(other) = set.cast_subtype(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    if let Some(repr) = structure.try_sub(other.to_elem(), self.to_elem()) {
                        Ok(set.from_elem(repr).into_py_any(py)?)
                    } else {
                        Err(PyValueError::new_err(""))
                    }
                } else {
                    Ok(py.NotImplemented())
                }
            }
        }
    }
}

#[macro_export]
macro_rules! impl_pymethods_mul {
    ($python_type:ident) => {
        #[pymethods]
        impl $python_type {
            fn __mul__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
                use ::algebraeon::rings::structure::MultiplicationSignature;
                static_assertions::const_assert!(impls::impls!($python_type : PythonElement));
                static_assertions::const_assert!(
                    impls::impls!(<$python_type as $crate::PythonElement>::Structure : MultiplicationSignature)
                );
                let py = other.py();
                let set = self.set();
                if let Ok(other) = set.cast_subtype(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    Ok(set.from_elem(structure.mul(self.to_elem(), other.to_elem())).into_py_any(py)?)
                } else {
                    Ok(py.NotImplemented())
                }
            }

            fn __rmul__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
                use ::algebraeon::rings::structure::MultiplicationSignature;
                static_assertions::const_assert!(impls::impls!($python_type : PythonElement));
                static_assertions::const_assert!(
                    impls::impls!(<$python_type as $crate::PythonElement>::Structure : MultiplicationSignature)
                );
                let py = other.py();
                let set = self.set();
                if let Ok(other) = set.cast_subtype(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    Ok(set.from_elem(structure.mul(other.to_elem(), self.to_elem())).into_py_any(py)?)
                } else {
                    Ok(py.NotImplemented())
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_pymethods_div {
    ($python_type:ident) => {
        #[pymethods]
        impl $python_type {
            fn __truediv__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
                use ::algebraeon::rings::structure::CancellativeMultiplicationSignature;
                static_assertions::const_assert!(impls::impls!($python_type : PythonElement));
                static_assertions::const_assert!(
                    impls::impls!(<$python_type as $crate::PythonElement>::Structure : CancellativeMultiplicationSignature)
                );
                let py = other.py();
                let set = self.set();
                if let Ok(other) = set.cast_subtype(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    match structure.try_divide(self.to_elem(), other.to_elem()) {
                        Some(repr) => Ok(set.from_elem(repr).into_py_any(py)?),
                        None => Err(PyValueError::new_err(format!(
                                "`{}` is not divisible by `{}`",
                                self.__repr__(),
                                other.__repr__()
                            ))
                        ),
                    }
                } else {
                    Ok(py.NotImplemented())
                }
            }

            fn __rtruediv__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
                use ::algebraeon::rings::structure::CancellativeMultiplicationSignature;
                static_assertions::const_assert!(impls::impls!($python_type : PythonElement));
                static_assertions::const_assert!(
                    impls::impls!(<$python_type as $crate::PythonElement>::Structure : CancellativeMultiplicationSignature)
                );
                let py = other.py();
                let set = self.set();
                if let Ok(other) = set.cast_subtype(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    match structure.try_divide(other.to_elem(), self.to_elem()) {
                        Some(repr) => Ok(set.from_elem(repr).into_py_any(py)?),
                        None => Err(PyValueError::new_err(format!(
                                "`{}` is not divisible by `{}`",
                                self.__repr__(),
                                other.__repr__()
                            ))),
                    }
                } else {
                    Ok(py.NotImplemented())
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_pymethods_nat_pow {
    ($python_type:ident) => {
        #[pymethods]
        impl $python_type {
            fn __pow__<'py>(
                &self,
                other: &Bound<'py, PyAny>,
                modulus: &Bound<'py, PyAny>,
            ) -> PyResult<Py<PyAny>> {
                use ::algebraeon::rings::structure::MultiplicativeMonoidSignature;
                use $crate::natural::PythonNatural;
                static_assertions::const_assert!(impls::impls!($python_type : PythonElement));
                static_assertions::const_assert!(
                    impls::impls!(<$python_type as $crate::PythonElement>::Structure : MultiplicativeMonoidSignature)
                );
                let py = other.py();
                let set = self.set();
                if !modulus.is_none() {
                    Ok(py.NotImplemented())
                } else {
                    if let Ok(other) = PythonNatural::py_new(other) {
                        Ok(set.from_elem(self.structure().nat_pow(self.to_elem(), other.to_elem())).into_py_any(py)?)
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

pub mod integer;
pub mod integer_factored;
pub mod integer_modulo;
pub mod integer_polynomial;
pub mod integer_polynomial_factored;
pub mod natural;
pub mod natural_factored;
pub mod natural_polynomial;
pub mod rational;
pub mod rational_polynomial;
