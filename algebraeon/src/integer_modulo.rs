use crate::PythonElement;
use crate::PythonElementCast;
use crate::PythonSet;
use crate::PythonStructure;
use crate::algebraeon_to_bignum_int;
use crate::bignum_to_algebraeon_int;
use crate::integer::PythonInteger;
use crate::integer::PythonIntegerSet;
use crate::natural::PythonNatural;
use algebraeon::nzq::Integer;
use algebraeon::nzq::IntegerCanonicalStructure;
use algebraeon::nzq::Natural;
use algebraeon::rings::natural::factorization::primes::is_prime_nat;
use algebraeon::rings::structure::EuclideanRemainderQuotientStructure;
use algebraeon::rings::structure::RingToQuotientFieldSignature;
use algebraeon::rings::structure::RingToQuotientRingSignature;
use algebraeon::sets::structure::MetaType;
use algebraeon::sets::structure::SetSignature;
use num_bigint::BigInt;
use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::{IntoPyObjectExt, exceptions::PyTypeError, prelude::*};

#[derive(Debug, Clone, PartialEq, Eq)]
enum IntegerModuloSet {
    Ring(
        EuclideanRemainderQuotientStructure<
            IntegerCanonicalStructure,
            IntegerCanonicalStructure,
            false,
        >,
    ),
    Field(
        EuclideanRemainderQuotientStructure<
            IntegerCanonicalStructure,
            IntegerCanonicalStructure,
            true,
        >,
    ),
}

impl IntegerModuloSet {
    fn new(n: Natural) -> Self {
        if is_prime_nat(&n) {
            Self::Field(Integer::structure().into_quotient_field_unchecked(n.into()))
        } else {
            Self::Ring(Integer::structure().into_quotient_ring(n.into()))
        }
    }

    fn n(&self) -> Natural {
        match self {
            IntegerModuloSet::Ring(structure) => Natural::try_from(structure.modulus()).unwrap(),
            IntegerModuloSet::Field(structure) => Natural::try_from(structure.modulus()).unwrap(),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PythonIntegerModuloSet {
    structure: IntegerModuloSet,
}

impl PythonSet for PythonIntegerModuloSet {
    type Elem = PythonIntegerModulo;

    fn str(&self) -> String {
        format!("ℤ/{}ℤ", self.structure.n())
    }

    fn repr(&self) -> String {
        format!("IntMod({})", self.structure.n())
    }
}

impl_pymethods_set!(PythonIntegerModuloSet);

#[pyclass]
#[derive(Debug, Clone)]
pub struct PythonIntegerModulo {
    pub repr: Integer,
    pub modulus: Natural,
}

impl PythonElement for PythonIntegerModulo {
    type Set = PythonIntegerModuloSet;

    fn set(&self) -> Self::Set {
        PythonIntegerModuloSet {
            structure: IntegerModuloSet::new(self.modulus.clone()),
        }
    }

    fn str(&self) -> String {
        format!("{}", self.repr)
    }

    fn repr(&self) -> String {
        format!("IntMod({} mod {})", self.repr, self.modulus)
    }
}

impl<'py> PythonElementCast<'py> for PythonIntegerModuloSet {
    fn cast_equiv(&self, obj: &Bound<'py, PyAny>) -> PyResult<PythonIntegerModulo> {
        Err(PyTypeError::new_err(format!(
            "Can't create an `IntMod` from a `{}`",
            obj.get_type().repr()?
        )))
    }

    fn cast_proper_subtype(&self, obj: &Bound<'py, PyAny>) -> Option<PythonIntegerModulo> {
        if let Ok(n) = PythonIntegerSet::default().cast_subtype(obj) {
            Some(PythonIntegerModulo {
                repr: Integer::from(n.inner()),
                modulus: Natural::ZERO,
            })
        } else {
            None
        }
    }
}

impl PythonStructure for PythonIntegerModulo {
    type Structure = EuclideanRemainderQuotientStructure<
        IntegerCanonicalStructure,
        IntegerCanonicalStructure,
        false,
    >;

    fn structure(&self) -> Self::Structure {
        Integer::structure().into_quotient_ring(self.modulus.clone().into())
    }

    fn inner(&self) -> &<Self::Structure as SetSignature>::Set {
        &self.repr
    }

    fn into_inner(self) -> <Self::Structure as SetSignature>::Set {
        self.repr
    }
}

// impl_pymethods_elem!(PythonIntegerModulo);
// impl_pymethods_cmp!(PythonIntegerModulo);
// impl_pymethods_pos!(PythonIntegerModulo);
// impl_pymethods_add!(PythonIntegerModulo);
// impl_pymethods_neg!(PythonIntegerModulo);
// impl_pymethods_sub!(PythonIntegerModulo);
// impl_pymethods_mul!(PythonIntegerModulo);
// impl_pymethods_div!(PythonIntegerModulo);
// impl_pymethods_nat_pow!(PythonIntegerModulo);

// #[pymethods]
// impl PythonInteger {
//     #[new]
//     pub fn py_new<'py>(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
//         Self::cast_subtype(obj)
//     }

//     pub fn __int__(&self) -> BigInt {
//         algebraeon_to_bignum_int(&self.inner)
//     }
// }

#[pymethods]
impl PythonIntegerSet {
    pub fn r#mod<'py>(&self, obj: &Bound<'py, PyAny>) -> PyResult<PythonIntegerModuloSet> {
        let obj = PythonNatural::py_new(obj)?;
        Ok(PythonIntegerModuloSet {
            structure: IntegerModuloSet::new(obj.inner),
        })
    }
}
