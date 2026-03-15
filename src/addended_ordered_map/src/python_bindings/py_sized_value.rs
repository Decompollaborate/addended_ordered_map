/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use alloc::sync::Arc;

use pyo3::exceptions::{PyNotImplementedError, PyRuntimeError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};

use crate::fallible::SizedValueFallible;
use crate::python_bindings::py_alias::PyS;

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

impl SizedValueFallible<PyS> for Py<PySizedValueBase> {
    type E = PyErr;

    fn size(&self) -> Result<PyS, Self::E> {
        Python::try_attach(|py| {
            let size = self.call_method0(py, "get_size")?;
            let s: u64 = size.extract(py)?;
            Ok(s)
        })
        .ok_or_else(|| PyRuntimeError::new_err("Error when adquiring the GIL"))
        .flatten()
    }
}

impl SizedValueFallible<PyS> for Arc<Py<PySizedValueBase>> {
    type E = PyErr;

    fn size(&self) -> Result<PyS, Self::E> {
        Arc::as_ref(self).size()
    }
}
