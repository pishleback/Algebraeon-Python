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
                let py = other.py();
                if let Ok(other) = Self::py_new(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    let eq_result = structure.equal(self.inner(), other.inner());
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
                let py = other.py();
                if let Ok(other) = Self::cast_subtype(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    let cmp_result = structure.cmp(self.inner(), other.inner());
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
                use ::algebraeon::rings::structure::AdditiveMonoidSignature;
                let py = other.py();
                if let Ok(other) = Self::cast_subtype(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    Ok(Self {
                        inner: structure.add(self.inner(), other.inner()),
                    }
                    .into_py_any(py)?)
                } else {
                    Ok(py.NotImplemented())
                }
            }

            fn __radd__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
                use ::algebraeon::rings::structure::AdditiveMonoidSignature;
                let py = other.py();
                if let Ok(other) = Self::cast_subtype(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    Ok(Self {
                        inner: structure.add(other.inner(), self.inner()),
                    }
                    .into_py_any(py)?)
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
                Self {
                    inner: self.inner().clone(),
                }
                .into_py_any(py)
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
                Self {
                    inner: self.structure().neg(self.inner()),
                }
                .into_py_any(py)
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
                let py = other.py();
                if let Ok(other) = Self::cast_subtype(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    Ok(Self {
                        inner: structure.sub(self.inner(), other.inner()),
                    }
                    .into_py_any(py)?)
                } else {
                    Ok(py.NotImplemented())
                }
            }

            fn __rsub__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
                use ::algebraeon::rings::structure::AdditiveGroupSignature;
                let py = other.py();
                if let Ok(other) = Self::cast_subtype(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    Ok(Self {
                        inner: structure.sub(other.inner(), self.inner()),
                    }
                    .into_py_any(py)?)
                } else {
                    Ok(py.NotImplemented())
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_pymethods_mul {
    ($python_type:ident) => {
        #[pymethods]
        impl $python_type {
            fn __mul__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
                use ::algebraeon::rings::structure::SemiRingSignature;
                let py = other.py();
                if let Ok(other) = Self::cast_subtype(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    Ok(Self {
                        inner: structure.mul(self.inner(), other.inner()),
                    }
                    .into_py_any(py)?)
                } else {
                    Ok(py.NotImplemented())
                }
            }

            fn __rmul__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
                use ::algebraeon::rings::structure::SemiRingSignature;
                let py = other.py();
                if let Ok(other) = Self::cast_subtype(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    Ok(Self {
                        inner: structure.mul(other.inner(), self.inner()),
                    }
                    .into_py_any(py)?)
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
                use ::algebraeon::rings::structure::IntegralDomainSignature;
                use ::algebraeon::rings::structure::RingDivisionError;
                let py = other.py();
                if let Ok(other) = Self::cast_subtype(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    match structure.div(self.inner(), other.inner()) {
                        Ok(result) => Ok(Self { inner: result }.into_py_any(py)?),
                        Err(e) => match e {
                            RingDivisionError::DivideByZero => {
                                Err(PyZeroDivisionError::new_err(format!(
                                    "when dividing `{}` by `{}`",
                                    self.__repr__(),
                                    other.__repr__()
                                )))
                            }
                            RingDivisionError::NotDivisible => Err(PyValueError::new_err(format!(
                                "`{}` is not divisible by `{}`",
                                self.__repr__(),
                                other.__repr__()
                            ))),
                        },
                    }
                } else {
                    Ok(py.NotImplemented())
                }
            }

            fn __rtruediv__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<Py<PyAny>> {
                use ::algebraeon::rings::structure::IntegralDomainSignature;
                use ::algebraeon::rings::structure::RingDivisionError;
                let py = other.py();
                if let Ok(other) = Self::cast_subtype(other) {
                    let structure = self.structure();
                    debug_assert_eq!(structure, other.structure());
                    match structure.div(other.inner(), self.inner()) {
                        Ok(result) => Ok(Self { inner: result }.into_py_any(py)?),
                        Err(e) => match e {
                            RingDivisionError::DivideByZero => {
                                Err(PyZeroDivisionError::new_err(format!(
                                    "when dividing `{}` by `{}`",
                                    self.__repr__(),
                                    other.__repr__()
                                )))
                            }
                            RingDivisionError::NotDivisible => Err(PyValueError::new_err(format!(
                                "`{}` is not divisible by `{}`",
                                self.__repr__(),
                                other.__repr__()
                            ))),
                        },
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
                use ::algebraeon::rings::structure::SemiRingSignature;
                use $crate::natural::PythonNatural;
                let py = other.py();
                if !modulus.is_none() {
                    Ok(py.NotImplemented())
                } else {
                    if let Ok(other) = PythonNatural::py_new(other) {
                        Ok(Self {
                            inner: self.structure().nat_pow(self.inner(), other.inner()),
                        }
                        .into_py_any(py)?)
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
