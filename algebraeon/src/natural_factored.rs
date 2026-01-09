use crate::algebraeon_to_bignum_nat;
use algebraeon::nzq::Natural;
use algebraeon::rings::structure::{Factored, MetaFactoringMonoid};
use pyo3::types::{PyDict, PyList};
use pyo3::{IntoPyObjectExt, prelude::*};

#[pyclass(name = "NatFactored")]
#[derive(Clone)]
pub struct PythonNaturalFactored {
    inner: Factored<Natural, Natural>,
}

impl PythonNaturalFactored {
    pub fn from_nat(n: &Natural) -> Self {
        Self {
            inner: n.clone().factor(),
        }
    }
}

#[pymethods]
impl PythonNaturalFactored {
    pub fn __str__(&self) -> String {
        if let Some(powers) = self.inner.powers() {
            if powers.is_empty() {
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
            }
        } else {
            "0".to_string()
        }
    }

    pub fn __repr__(&self) -> String {
        format!("NatFactored({})", self.__str__())
    }

    pub fn is_prime(&self) -> bool {
        if let Some(factors) = self.inner.powers() {
            if factors.len() == 1 {
                factors[0].1 == Natural::ONE
            } else {
                false
            }
        } else {
            false
        }
    }

    /// A dict of the prime factors pointing at their non-zero powers.
    ///
    /// None if 0
    pub fn powers<'py>(&self, py: Python<'py>) -> Py<PyAny> {
        if let Some(factors) = self.inner.powers() {
            let dict = PyDict::new(py);
            for (p, k) in factors {
                dict.set_item(algebraeon_to_bignum_nat(p), algebraeon_to_bignum_nat(k))
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
                            ps.push(algebraeon_to_bignum_nat(p));
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
                    .map(|(p, _)| algebraeon_to_bignum_nat(p))
                    .collect::<Vec<_>>(),
            )
            .unwrap()
            .into_py_any(py)
            .unwrap()
        } else {
            py.None()
        }
    }
}
