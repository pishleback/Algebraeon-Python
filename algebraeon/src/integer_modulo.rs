use crate::PythonElement;
use crate::PythonElementCast;
use crate::PythonSet;
use crate::integer::PythonInteger;
use crate::integer::PythonIntegerSet;
use crate::natural::PythonNatural;
use algebraeon::nzq::Integer;
use algebraeon::nzq::IntegerCanonicalStructure;
use algebraeon::nzq::Natural;
use algebraeon::rings::structure::EuclideanRemainderQuotientStructure;
use algebraeon::rings::structure::MetaEuclideanDivisionSignature;
use algebraeon::rings::structure::MultiplicativeMonoidSignature;
use algebraeon::rings::structure::MultiplicativeMonoidTryInverseSignature;
use algebraeon::rings::structure::RingToQuotientFieldSignature;
use algebraeon::rings::structure::RingToQuotientRingSignature;
use algebraeon::sets::structure::MetaType;
use algebraeon::sets::structure::SetSignature;
use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::{IntoPyObjectExt, exceptions::PyTypeError, prelude::*};

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PythonIntegerModuloSet {
    structure: EuclideanRemainderQuotientStructure<
        IntegerCanonicalStructure,
        IntegerCanonicalStructure,
        false,
    >,
}

impl PythonIntegerModuloSet {
    fn new(n: Natural) -> Self {
        Self {
            structure: Integer::structure().into_quotient_ring(n.into()).unwrap(),
        }
    }

    fn n(&self) -> Natural {
        Natural::try_from(self.structure.modulus()).unwrap()
    }
}

impl PythonSet for PythonIntegerModuloSet {
    type Elem = PythonIntegerModulo;

    fn from_elem(&self, elem: Integer) -> Self::Elem {
        PythonIntegerModulo {
            repr: elem,
            modulus: self.n(),
        }
    }

    fn str(&self) -> String {
        format!("ℤ/{}ℤ", self.n())
    }

    fn repr(&self) -> String {
        format!("IntMod({})", self.n())
    }
}

impl_pymethods_set!(PythonIntegerModuloSet);

#[pyclass]
#[derive(Debug, Clone)]
pub struct PythonIntegerModulo {
    pub repr: Integer,
    pub modulus: Natural,
}

impl PythonIntegerModulo {
    pub fn ring_structure(
        &self,
    ) -> EuclideanRemainderQuotientStructure<
        IntegerCanonicalStructure,
        IntegerCanonicalStructure,
        false,
    > {
        Integer::structure()
            .into_quotient_ring(self.modulus.clone().into())
            .unwrap()
    }

    pub fn try_field_structure(
        &self,
    ) -> Option<
        EuclideanRemainderQuotientStructure<
            IntegerCanonicalStructure,
            IntegerCanonicalStructure,
            true,
        >,
    > {
        Integer::structure().into_quotient_field(self.modulus.clone().into())
    }
}

impl PythonElement for PythonIntegerModulo {
    type Set = PythonIntegerModuloSet;

    type Structure = EuclideanRemainderQuotientStructure<
        IntegerCanonicalStructure,
        IntegerCanonicalStructure,
        false,
    >;

    fn structure(&self) -> Self::Structure {
        self.ring_structure()
    }

    fn to_elem(&self) -> &<Self::Structure as SetSignature>::Set {
        &self.repr
    }

    fn into_elem(self) -> <Self::Structure as SetSignature>::Set {
        self.repr
    }

    fn set(&self) -> Self::Set {
        PythonIntegerModuloSet {
            structure: Integer::structure()
                .into_quotient_ring(self.modulus.clone().into())
                .unwrap(),
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
    fn cast_exact(&self, obj: &Bound<'py, PyAny>) -> Option<Self::Elem> {
        if let Ok(mut other) = obj.extract::<Self::Elem>() {
            let n = self.n();
            if other.modulus.rem(&n) == Natural::ZERO {
                other.modulus = n;
                Some(other)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn cast_equiv(&self, obj: &Bound<'py, PyAny>) -> PyResult<PythonIntegerModulo> {
        Err(PyTypeError::new_err(format!(
            "Can't create an `IntMod` from a `{}`",
            obj.get_type().repr()?
        )))
    }

    fn cast_proper_subtype(&self, obj: &Bound<'py, PyAny>) -> Option<PythonIntegerModulo> {
        if let Ok(n) = PythonIntegerSet::default().cast_subtype(obj) {
            Some(PythonIntegerModulo {
                repr: Integer::from(n.to_elem()),
                modulus: self.n(),
            })
        } else {
            None
        }
    }
}

impl_pymethods_elem!(PythonIntegerModulo);
impl_pymethods_eq!(PythonIntegerModulo);
impl_pymethods_pos!(PythonIntegerModulo);
impl_pymethods_add!(PythonIntegerModulo);
impl_pymethods_neg!(PythonIntegerModulo);
impl_pymethods_sub!(PythonIntegerModulo);
impl_pymethods_mul!(PythonIntegerModulo);

#[pymethods]
impl PythonIntegerModulo {
    fn __pow__<'py>(
        &self,
        other: &Bound<'py, PyAny>,
        modulus: &Bound<'py, PyAny>,
    ) -> PyResult<Py<PyAny>> {
        let py = other.py();
        let set = self.set();
        if !modulus.is_none() {
            Ok(py.NotImplemented())
        } else if let Ok(other) = PythonNatural::py_new(other) {
            set.from_elem(
                self.ring_structure()
                    .nat_pow(self.to_elem(), other.to_elem()),
            )
            .into_py_any(py)
        } else if let Ok(other) = PythonInteger::py_new(other) {
            if let Some(repr) = self
                .ring_structure()
                .try_int_pow(self.to_elem(), other.to_elem())
            {
                set.from_elem(repr).into_py_any(py)
            } else {
                Err(PyValueError::new_err(format!(
                    "can't invert `{}` as it is not coprime to the modulus `{}`",
                    self.to_elem(),
                    other.to_elem()
                )))
            }
        } else {
            Ok(py.NotImplemented())
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

#[pymethods]
impl PythonIntegerModulo {
    #[new]
    pub fn py_new<'py>(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        PythonIntegerModuloSet::new(Natural::ZERO).cast_subtype(obj)
    }
}

#[pymethods]
impl PythonIntegerSet {
    pub fn r#mod<'py>(&self, obj: &Bound<'py, PyAny>) -> PyResult<PythonIntegerModuloSet> {
        let obj = PythonNatural::py_new(obj)?;
        Ok(PythonIntegerModuloSet::new(obj.inner))
    }
}
