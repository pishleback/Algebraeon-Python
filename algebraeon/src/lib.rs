use ::algebraeon::{
    nzq::{Integer, Natural, Rational},
    rings::polynomial::Polynomial,
};
use num_bigint::{BigInt, BigUint};
use pyo3::{
    IntoPyObjectExt,
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
};

#[pymodule]
fn algebraeon(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PythonNatural>()?;
    m.add_class::<PythonInteger>()?;
    m.add_class::<PythonRational>()?;
    m.add_class::<PythonPolynomial>()?;

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

trait Cast: Sized {
    fn frompy<'py>(obj: &Bound<'py, PyAny>) -> PyResult<Self>;
}

impl Cast for Natural {
    fn frompy<'py>(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(PythonNatural::py_new(obj)?.inner)
    }
}

impl Cast for Integer {
    fn frompy<'py>(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(PythonInteger::py_new(obj)?.inner)
    }
}

impl Cast for Rational {
    fn frompy<'py>(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(PythonRational::py_new(obj)?.inner)
    }
}

trait PythonCast<'py>: Sized + for<'a> FromPyObject<'a, 'py> {
    fn zero() -> Option<Self>;

    fn frompy_eq_impl(obj: &Bound<'py, PyAny>) -> PyResult<Self>;

    fn frompy_lt_impl(obj: &Bound<'py, PyAny>) -> PyResult<Self>;

    fn frompy_gt_impl(obj: &Bound<'py, PyAny>) -> PyResult<Self>;

    fn frompy_lt_common(_obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Err(PyTypeError::new_err(""))
    }

    fn frompy_gt_common(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        let py = obj.py();
        if let Ok(obj) = PythonPolynomial::frompy_ge(obj) {
            if obj.inner.is_empty()
                && let Some(zero) = Self::zero()
            {
                Ok(zero)
            } else if obj.inner.len() == 1
                && let Ok(obj) = Self::frompy(obj.inner[0].bind(py))
            {
                Ok(obj)
            } else {
                Err(PyTypeError::new_err(""))
            }
        } else {
            Err(PyTypeError::new_err(""))
        }
    }

    fn frompy_eq_common(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(obj) = obj.extract::<Self>() {
            Ok(obj)
        } else {
            Err(PyTypeError::new_err(""))
        }
    }

    fn frompy_lt(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(obj) = Self::frompy_lt_common(obj) {
            Ok(obj)
        } else {
            Ok(Self::frompy_lt_impl(obj)?)
        }
    }

    fn frompy_gt(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(obj) = Self::frompy_gt_common(obj) {
            Ok(obj)
        } else {
            Ok(Self::frompy_gt_impl(obj)?)
        }
    }

    fn frompy_eq(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(obj) = Self::frompy_eq_common(obj) {
            Ok(obj)
        } else {
            Ok(Self::frompy_eq_impl(obj)?)
        }
    }

    fn frompy_le(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(obj) = Self::frompy_eq(obj) {
            Ok(obj)
        } else {
            Ok(Self::frompy_lt_impl(obj)?)
        }
    }

    fn frompy_ge(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(obj) = Self::frompy_eq(obj) {
            Ok(obj)
        } else {
            Ok(Self::frompy_gt_impl(obj)?)
        }
    }

    fn frompy(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(obj) = Self::frompy_eq(obj) {
            Ok(obj)
        } else if let Ok(obj) = Self::frompy_lt(obj) {
            Ok(obj)
        } else {
            Ok(Self::frompy_gt(obj)?)
        }
    }
}

#[pyclass(name = "Nat")]
#[derive(Clone)]
pub struct PythonNatural {
    inner: Natural,
}

#[pyclass(name = "Int")]
#[derive(Clone)]
pub struct PythonInteger {
    inner: Integer,
}

#[pyclass(name = "Rat")]
#[derive(Clone)]
pub struct PythonRational {
    inner: Rational,
}

impl<'py> PythonCast<'py> for PythonNatural {
    fn zero() -> Option<Self> {
        Some(PythonNatural {
            inner: Natural::ZERO,
        })
    }

    fn frompy_eq_impl(_obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Err(PyTypeError::new_err(""))
    }

    fn frompy_lt_impl(_obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Err(PyTypeError::new_err(""))
    }

    fn frompy_gt_impl(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        let obj = PythonInteger::frompy_ge(obj)?;
        if let Ok(n) = Natural::try_from(obj.inner) {
            Ok(Self { inner: n })
        } else {
            Err(PyValueError::new_err(""))
        }
    }
}

impl<'py> PythonCast<'py> for PythonInteger {
    fn zero() -> Option<Self> {
        Some(Self {
            inner: Integer::ZERO,
        })
    }

    fn frompy_eq_impl(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            inner: bignum_to_algebraeon_int(&obj.extract::<BigInt>()?),
        })
    }

    fn frompy_lt_impl(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            inner: Integer::from(PythonNatural::frompy_le(obj)?.inner),
        })
    }

    fn frompy_gt_impl(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        let obj = PythonRational::frompy_ge(obj)?;
        if let Ok(n) = Integer::try_from(obj.inner) {
            Ok(Self { inner: n })
        } else {
            Err(PyValueError::new_err(""))
        }
    }
}

impl<'py> PythonCast<'py> for PythonRational {
    fn zero() -> Option<Self> {
        Some(Self {
            inner: Rational::ZERO,
        })
    }

    fn frompy_eq_impl(_obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Err(PyTypeError::new_err(""))
    }

    fn frompy_lt_impl(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            inner: Rational::from(PythonInteger::frompy_le(obj)?.inner),
        })
    }

    fn frompy_gt_impl(_obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Err(PyTypeError::new_err(""))
    }
}

#[pymethods]
impl PythonNatural {
    #[new]
    fn py_new<'py>(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Self::frompy(obj)
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

#[pymethods]
impl PythonInteger {
    #[new]
    fn py_new<'py>(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Self::frompy(obj)
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

#[pymethods]
impl PythonRational {
    #[new]
    fn py_new<'py>(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Self::frompy(obj)
    }

    fn __int__(&self) -> BigInt {
        todo!()
    }

    fn __str__(&self) -> String {
        format!("{}", self.inner)
    }

    fn __repr__(&self) -> String {
        format!("Rat({})", self.inner)
    }
}

impl<T: Cast> Cast for Polynomial<T> {
    fn frompy<'py>(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        let py = obj.py();
        if let Ok(poly) = PythonPolynomial::py_new(obj) {
            let mut coeffs = vec![];
            for coeff in poly.inner {
                if let Ok(coeff) = T::frompy(coeff.bind(py)) {
                    coeffs.push(coeff);
                } else {
                    return Err(PyTypeError::new_err(format!(
                        "Can't cast coefficient {}",
                        coeff.bind(py).repr()?
                    )));
                }
            }
            Ok(Polynomial::from_coeffs(coeffs))
        } else {
            Err(PyTypeError::new_err(format!(
                "Can't create a polynomial from `{}`",
                obj.repr()?
            )))
        }
    }
}

#[pyclass(name = "Poly")]
#[derive(Debug, Clone)]
pub struct PythonPolynomial {
    inner: Vec<Py<PyAny>>,
}

impl<'py> PythonCast<'py> for PythonPolynomial {
    fn zero() -> Option<Self> {
        Some(Self { inner: vec![] })
    }

    fn frompy_eq_impl(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(obj) = obj.extract::<Vec<Py<PyAny>>>() {
            Ok(Self { inner: obj })
        } else {
            Err(PyTypeError::new_err(""))
        }
    }

    fn frompy_lt_impl(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            inner: vec![obj.clone().into()],
        })
    }

    fn frompy_gt_impl(_obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Err(PyTypeError::new_err(""))
    }
}

#[pymethods]
impl PythonPolynomial {
    #[new]
    fn py_new<'py>(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Self::frompy(obj)
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!("Poly({})", {
            let mut coeffs = "".to_string();
            coeffs += "[";
            for i in 0..self.inner.len() {
                if i != 0 {
                    coeffs += ", ";
                }
                coeffs += self.inner[i].bind(py).repr()?.to_str()?;
            }
            coeffs += "]";
            coeffs
        }))
    }

    fn to_nat_poly(&self, py: Python<'_>) -> PyResult<PythonPolynomial> {
        Ok(Self {
            inner: Polynomial::<Natural>::frompy(self.clone().into_py_any(py)?.bind(py))?
                .into_coeffs()
                .into_iter()
                .map(|c| PythonNatural { inner: c }.into_py_any(py))
                .collect::<PyResult<_>>()?,
        })
    }

    
}
