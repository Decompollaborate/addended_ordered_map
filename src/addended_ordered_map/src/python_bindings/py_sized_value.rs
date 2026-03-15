/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use alloc::sync::Arc;

use pyo3::exceptions::PyNotImplementedError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};

use crate::python_bindings::py_alias::PyS;
use crate::SizedValue;

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

impl SizedValue<PyS> for Py<PySizedValueBase> {
    fn size(&self) -> PyS {
        Python::try_attach(|py| {
            let size = self.call_method0(py, "get_size").unwrap();
            size.extract(py).unwrap()
        })
        .unwrap()
    }
}

impl SizedValue<PyS> for Arc<Py<PySizedValueBase>> {
    fn size(&self) -> PyS {
        Arc::as_ref(self).size()
    }
}
