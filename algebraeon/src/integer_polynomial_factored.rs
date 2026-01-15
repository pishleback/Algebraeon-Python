use crate::integer_factored::PythonIntegerFactored;
use crate::integer_polynomial::PythonIntegerPolynomial;
use crate::{PythonStructure, algebraeon_to_bignum_nat};
use algebraeon::nzq::{Integer, Natural};
use algebraeon::rings::polynomial::{Polynomial, ToPolynomialSignature};
use algebraeon::rings::structure::{
    Factored, MetaCharZeroRingSignature, MetaFactoringMonoid, UniqueFactorizationMonoidSignature,
};
use algebraeon::sets::structure::MetaType;
use pyo3::types::PyList;
use pyo3::{IntoPyObjectExt, prelude::*};

#[pymethods]
impl PythonIntegerPolynomial {
    pub fn factor(&self) -> PythonIntegerPolynomialFactored {
        PythonIntegerPolynomialFactored {
            inner: self.inner().factor(),
        }
    }

    pub fn is_irreducible(&self) -> bool {
        self.factor().is_irreducible()
    }
}

#[pyclass(name = "IntPolynomialFactored")]
#[derive(Clone)]
pub struct PythonIntegerPolynomialFactored {
    inner: Factored<Polynomial<Integer>, Natural>,
}

#[pymethods]
impl PythonIntegerPolynomialFactored {
    pub fn __str__(&self) -> String {
        format!("{}", self.inner)
    }

    pub fn __repr__(&self) -> String {
        format!("IntPolynomialFactored({})", self.__str__())
    }

    pub fn is_irreducible(&self) -> bool {
        Integer::structure()
            .polynomials()
            .factorizations()
            .is_irreducible(&self.inner)
    }

    /// A dict of the prime factors pointing at their non-zero powers.
    ///
    /// None if 0
    pub fn powers<'py>(&self, py: Python<'py>) -> Py<PyAny> {
        if let Some(factors) = self.inner.powers() {
            factors
                .iter()
                .map(|(p, k)| {
                    (
                        PythonIntegerPolynomial { inner: p.clone() },
                        algebraeon_to_bignum_nat(k),
                    )
                })
                .collect::<Vec<_>>()
                .into_py_any(py)
                .unwrap()
        } else {
            py.None()
        }
    }

    /// A list of the irreducible factors with repetitions.
    ///
    /// None if 0
    pub fn irreducibles<'py>(&self, py: Python<'py>) -> Py<PyAny> {
        if let Some(factors) = self.inner.powers() {
            PyList::new(
                py,
                factors
                    .iter()
                    .flat_map(|(p, k)| {
                        let mut ps = vec![];
                        let mut k_count = Natural::ZERO;
                        while &k_count < k {
                            ps.push(PythonIntegerPolynomial { inner: p.clone() });
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

    /// A list of the irreducible factors without repetitions.
    ///
    /// None if 0
    pub fn distinct_irreducibles<'py>(&self, py: Python<'py>) -> Py<PyAny> {
        if let Some(factors) = self.inner.powers() {
            PyList::new(
                py,
                factors
                    .iter()
                    .map(|(p, _)| PythonIntegerPolynomial { inner: p.clone() })
                    .collect::<Vec<_>>(),
            )
            .unwrap()
            .into_py_any(py)
            .unwrap()
        } else {
            py.None()
        }
    }

    /// The polynomial part of the factorization
    pub fn primitive(&self) -> PythonIntegerPolynomialFactored {
        if let Some(powers) = self.inner.powers() {
            PythonIntegerPolynomialFactored {
                inner: Integer::structure()
                    .polynomials()
                    .factorizations()
                    .new_powers_unchecked(
                        powers
                            .iter()
                            .filter(|(p, _)| p.try_to_int().is_none())
                            .cloned()
                            .collect(),
                    ),
            }
        } else {
            PythonIntegerPolynomialFactored {
                inner: Factored::Zero,
            }
        }
    }

    /// The integer part of the factorization including sign
    pub fn content(&self) -> PythonIntegerFactored {
        if let Some((unit, powers)) = self.inner.unit_and_powers() {
            PythonIntegerFactored {
                inner: Integer::structure()
                    .factorizations()
                    .new_unit_and_powers_unchecked(
                        unit.try_to_int().unwrap(),
                        powers
                            .iter()
                            .filter_map(|(p, k)| Some((p.try_to_int()?, k.clone())))
                            .collect(),
                    ),
            }
        } else {
            PythonIntegerFactored {
                inner: Factored::Zero,
            }
        }
    }
}
