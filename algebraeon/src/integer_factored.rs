use crate::integer::PythonInteger;
use crate::{algebraeon_to_bignum_int, algebraeon_to_bignum_nat};
use algebraeon::nzq::{Integer, Natural};
use algebraeon::rings::structure::{
    Factored, MetaFactoringMonoid, UniqueFactorizationMonoidSignature,
};
use algebraeon::sets::structure::MetaType;
use pyo3::types::{PyDict, PyList};
use pyo3::{IntoPyObjectExt, prelude::*};

#[pymethods]
impl PythonInteger {
    pub fn factor(&self) -> PythonIntegerFactored {
        PythonIntegerFactored::from_int(&self.inner)
    }

    pub fn is_prime(&self) -> bool {
        self.factor().is_prime()
    }
}

#[pyclass(name = "IntFactored")]
#[derive(Clone)]
pub struct PythonIntegerFactored {
    pub inner: Factored<Integer, Natural>,
}

impl PythonIntegerFactored {
    pub fn from_int(n: &Integer) -> Self {
        Self {
            inner: n.clone().factor(),
        }
    }
}

#[pymethods]
impl PythonIntegerFactored {
    pub fn __str__(&self) -> String {
        if let Some((unit, powers)) = self.inner.unit_and_powers() {
            let s = if powers.is_empty() {
                "1".to_string()
            } else {
                powers
                    .iter()
                    .map(|(p, k)| {
                        if k == &Natural::ONE {
                            format!("{p}")
                        } else {
                            format!("{p}^{k}")
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" × ")
            };
            if *unit == Integer::ONE {
                s
            } else if -unit == Integer::ONE {
                format!("- {s}")
            } else {
                unreachable!()
            }
        } else {
            "0".to_string()
        }
    }

    pub fn __repr__(&self) -> String {
        format!("NatFactored({})", self.__str__())
    }

    pub fn is_prime(&self) -> bool {
        Integer::structure()
            .factorizations()
            .is_irreducible(&self.inner)
    }

    /// A dict of the prime factors pointing at their non-zero powers.
    ///
    /// None if 0
    pub fn powers<'py>(&self, py: Python<'py>) -> Py<PyAny> {
        if let Some(factors) = self.inner.powers() {
            let dict = PyDict::new(py);
            for (p, k) in factors {
                dict.set_item(algebraeon_to_bignum_int(p), algebraeon_to_bignum_nat(k))
                    .unwrap();
            }
            dict.into_py_any(py).unwrap()
        } else {
            py.None()
        }
    }

    /// A list of the prime factors with repetitions.
    ///
    /// None if 0
    pub fn primes<'py>(&self, py: Python<'py>) -> Py<PyAny> {
        if let Some(factors) = self.inner.powers() {
            PyList::new(
                py,
                factors
                    .iter()
                    .flat_map(|(p, k)| {
                        let mut ps = vec![];
                        let mut k_count = Natural::ZERO;
                        while &k_count < k {
                            ps.push(algebraeon_to_bignum_int(p));
                            k_count += Natural::ONE;
                        }
                        ps
                    })
                    .collect::<Vec<_>>(),
            )
            .unwrap()
            .into_py_any(py)
            .unwrap()
        } else {
            py.None()
        }
    }

    /// A list of the prime factors without repetitions.
    ///
    /// None if 0
    pub fn distinct_primes<'py>(&self, py: Python<'py>) -> Py<PyAny> {
        if let Some(factors) = self.inner.powers() {
            PyList::new(
                py,
                factors
                    .iter()
                    .map(|(p, _)| algebraeon_to_bignum_int(p))
                    .collect::<Vec<_>>(),
            )
            .unwrap()
            .into_py_any(py)
            .unwrap()
        } else {
            py.None()
        }
    }

    /// The sign, either -1, 0, or 1
    pub fn sign(&self) -> PythonInteger {
        match &self.inner {
            Factored::Zero => PythonInteger {
                inner: Integer::ZERO,
            },
            Factored::NonZero(nz) => {
                let u = nz.unit();
                debug_assert!(*u == Integer::ONE || *u == -Integer::ONE);
                PythonInteger { inner: u.clone() }
            }
        }
    }
}
