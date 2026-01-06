use crate::PythonElement;
use crate::PythonElementCast;
use crate::PythonPolynomialSet;
use crate::PythonSet;
use crate::PythonStructure;
use crate::PythonToPolynomialSet;
use crate::impl_pymethods_add;
use crate::impl_pymethods_div;
use crate::impl_pymethods_elem;
use crate::impl_pymethods_eq;
use crate::impl_pymethods_mul;
use crate::impl_pymethods_nat_pow;
use crate::impl_pymethods_neg;
use crate::impl_pymethods_pos;
use crate::impl_pymethods_set;
use crate::impl_pymethods_sub;
use crate::natural::PythonNatural;
use crate::natural::PythonNaturalSet;
use algebraeon::nzq::Natural;
use algebraeon::nzq::NaturalCanonicalStructure;
use algebraeon::rings::polynomial::Polynomial;
use algebraeon::rings::polynomial::PolynomialSemiRingStructure;
use algebraeon::rings::polynomial::PolynomialStructure;
use algebraeon::rings::polynomial::RingToPolynomialSignature;
use algebraeon::rings::polynomial::SemiRingToPolynomialSemiRingSignature;
use algebraeon::sets::structure::MetaType;
use algebraeon::sets::structure::SetSignature;
use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::exceptions::PyZeroDivisionError;
use pyo3::{IntoPyObjectExt, exceptions::PyTypeError, prelude::*};

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PythonNaturalPolynomialSet {}

impl PythonSet for PythonNaturalPolynomialSet {
    type Elem = PythonNaturalPolynomial;

    fn str(&self) -> String {
        format!("{}[λ]", PythonNaturalSet::default().str())
    }

    fn repr(&self) -> String {
        format!("Polynomial({})", PythonNaturalSet::default().repr())
    }
}

impl PythonPolynomialSet for PythonNaturalPolynomialSet {
    fn var(&self) -> <Self as PythonSet>::Elem {
        todo!()
        // PythonNaturalPolynomial {
        //     inner: Polynomial::var(),
        // }
    }
}

impl_pymethods_set!(PythonNaturalPolynomialSet);
impl_pymethods_polynomial_set!(PythonNaturalPolynomialSet);

impl PythonToPolynomialSet for PythonNaturalSet {
    type PolynomialSet = PythonNaturalPolynomialSet;

    fn polynomials(&self) -> Self::PolynomialSet {
        PythonNaturalPolynomialSet::default()
    }
}

impl_pymethods_to_polynomial_set!(PythonNaturalSet);

#[pyclass]
#[derive(Debug, Clone)]
pub struct PythonNaturalPolynomial {
    inner: Polynomial<Natural>,
}

impl PythonElement for PythonNaturalPolynomial {
    type Set = PythonNaturalPolynomialSet;

    fn set(&self) -> Self::Set {
        PythonNaturalPolynomialSet {}
    }

    fn str(&self) -> String {
        format!("{}", self.inner)
    }

    fn repr(&self) -> String {
        format!(
            "Polynomial({}, {})",
            self.inner,
            PythonNaturalSet::default().repr()
        )
    }
}

impl<'py> PythonElementCast<'py> for PythonNaturalPolynomial {
    fn cast_equiv(_obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Err(PyTypeError::new_err(""))
    }

    fn cast_proper_subtype(obj: &Bound<'py, PyAny>) -> Option<Self> {
        if let Ok(n) = PythonNatural::cast_subtype(obj) {
            Some(Self {
                inner: Polynomial::constant(n.inner().clone()),
            })
        } else {
            None
        }
    }
}

impl PythonStructure for PythonNaturalPolynomial {
    type Structure = PolynomialSemiRingStructure<NaturalCanonicalStructure, NaturalCanonicalStructure>;

    fn structure(&self) -> Self::Structure {
        Natural::structure().into_polynomial_semiring()
    }

    fn inner(&self) -> &<Self::Structure as SetSignature>::Set {
        &self.inner
    }
}

impl_pymethods_elem!(PythonNaturalPolynomial);
impl_pymethods_eq!(PythonNaturalPolynomial);
impl_pymethods_pos!(PythonNaturalPolynomial);
impl_pymethods_add!(PythonNaturalPolynomial);
impl_pymethods_mul!(PythonNaturalPolynomial);
impl_pymethods_nat_pow!(PythonNaturalPolynomial);

#[pymethods]
impl PythonNaturalPolynomial {
    #[new]
    pub fn py_new<'py>(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        Self::cast_subtype(obj)
    }
}
