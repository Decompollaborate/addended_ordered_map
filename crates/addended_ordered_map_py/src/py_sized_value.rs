/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use std::ops::Deref;
use std::sync::Arc;

use pyo3::exceptions::{PyNotImplementedError, PyRuntimeError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};

use addended_ordered_map::fallible::SizedValueFallible;

use crate::py_alias::PyS;

#[pyclass(
    name = "SizedValue",
    module = "addended_ordered_map",
    subclass,
    from_py_object
)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub struct PySizedValueBase;

#[pymethods]
impl PySizedValueBase {
    #[new]
    #[pyo3(signature = (*args, **kwargs))]
    pub fn __new__(
        #[expect(unused_variables)] args: &Bound<'_, PyTuple>,
        #[expect(unused_variables)] kwargs: Option<&Bound<'_, PyDict>>,
    ) -> Self {
        Self
    }

    pub fn get_size(&self) -> PyResult<PyS> {
        Err(PyNotImplementedError::new_err(
            "get_size must be implemented by subclass",
        ))
    }
}

#[derive(Debug)]
#[non_exhaustive]
#[repr(transparent)]
pub struct PySizedValueBaseWrapper(Py<PySizedValueBase>);

impl SizedValueFallible<PyS, PyErr> for PySizedValueBaseWrapper {
    fn size(&self) -> Result<PyS, PyErr> {
        Python::try_attach(|py| self.0.call_method0(py, "get_size"))
            .ok_or_else(|| PyRuntimeError::new_err("Error when adquiring the GIL"))
            .flatten()
    }
}

impl Deref for PySizedValueBaseWrapper {
    type Target = Py<PySizedValueBase>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
#[repr(transparent)]
pub struct PySizedValueBaseWrapperArc(Arc<PySizedValueBaseWrapper>);

impl PySizedValueBaseWrapperArc {
    pub fn new(value: Py<PySizedValueBase>) -> Self {
        Self(Arc::new(PySizedValueBaseWrapper(value)))
    }
}

impl SizedValueFallible<PyS, PyErr> for PySizedValueBaseWrapperArc {
    fn size(&self) -> Result<PyS, PyErr> {
        Arc::as_ref(&self.0).size()
    }
}

impl Deref for PySizedValueBaseWrapperArc {
    type Target = Arc<PySizedValueBaseWrapper>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
