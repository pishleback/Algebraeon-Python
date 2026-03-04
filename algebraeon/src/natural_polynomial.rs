use crate::CastError;
use crate::PythonElement;
use crate::PythonElementCast;
use crate::PythonPolynomialSet;
use crate::PythonSet;
use crate::PythonToPolynomialSet;
use crate::integer_polynomial::PythonIntegerPolynomialSet;
use crate::natural::PythonNaturalSet;
use algebraeon::nzq::Natural;
use algebraeon::nzq::NaturalCanonicalStructure;
use algebraeon::rings::polynomial::Polynomial;
use algebraeon::rings::polynomial::PolynomialStructure;
use algebraeon::rings::polynomial::ToPolynomialSignature;
use algebraeon::sets::structure::MetaType;
use algebraeon::sets::structure::SetSignature;
use pyo3::basic::CompareOp;
use pyo3::{IntoPyObjectExt, prelude::*};

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PythonNaturalPolynomialSet {}

impl PythonSet for PythonNaturalPolynomialSet {
    type Elem = PythonNaturalPolynomial;

    fn from_elem(&self, elem: Polynomial<Natural>) -> Self::Elem {
        PythonNaturalPolynomial { inner: elem }
    }

    fn str(&self) -> String {
        format!("{}[λ]", PythonNaturalSet::default().str())
    }

    fn repr(&self) -> String {
        format!("Poly({})", PythonNaturalSet::default().repr())
    }
}

impl PythonPolynomialSet for PythonNaturalPolynomialSet {
    fn var(&self) -> <Self as PythonSet>::Elem {
        // todo: use Polynomial::var()
        PythonNaturalPolynomial {
            inner: Polynomial::from_coeffs(vec![Natural::ZERO, Natural::ONE]),
        }
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

#[pyclass(name = "NatPoly")]
#[derive(Debug, Clone)]
pub struct PythonNaturalPolynomial {
    inner: Polynomial<Natural>,
}

impl PythonElement for PythonNaturalPolynomial {
    type Set = PythonNaturalPolynomialSet;

    type Structure = PolynomialStructure<NaturalCanonicalStructure, NaturalCanonicalStructure>;

    fn structure(&self) -> Self::Structure {
        Natural::structure().into_polynomials()
    }

    fn to_elem(&self) -> &<Self::Structure as SetSignature>::Set {
        &self.inner
    }

    fn into_elem(self) -> <Self::Structure as SetSignature>::Set {
        self.inner
    }

    fn set(&self) -> Self::Set {
        PythonNaturalPolynomialSet {}
    }

    fn str(&self) -> String {
        format!("{}", self.inner)
    }

    fn repr(&self) -> String {
        format!(
            "Poly({}, {})",
            self.inner,
            PythonNaturalSet::default().repr()
        )
    }
}

impl<'py> PythonElementCast<'py> for PythonNaturalPolynomialSet {
    fn proper_subset_cast_impl(&self, obj: &Bound<'py, PyAny>) -> Result<Self::Elem, CastError> {
        if let Ok(n) = PythonNaturalSet::default().implicit_cast(obj) {
            return Ok(PythonNaturalPolynomial {
                inner: Polynomial::constant(n.to_elem().clone()),
            });
        }
        Err(CastError::Type)
    }

    fn proper_supset_cast_impl(&self, obj: &Bound<'py, PyAny>) -> Result<Self::Elem, CastError> {
        if let Ok(n) = PythonIntegerPolynomialSet::default().supset_cast_impl(obj) {
            if let Ok(coeffs) = n
                .inner
                .coeffs()
                .map(Natural::try_from)
                .collect::<Result<Vec<_>, _>>()
            {
                return Ok(PythonNaturalPolynomial {
                    inner: Polynomial::from_coeffs(coeffs),
                });
            } else {
                return Err(CastError::Value);
            }
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
        PythonNaturalPolynomialSet::default().explicit_cast(obj)
    }
}
