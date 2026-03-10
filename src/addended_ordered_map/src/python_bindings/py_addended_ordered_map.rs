/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyInt;

use crate::{AddendedOrderedMap, FindSettings};
use crate::python_bindings::PySizedValueBase;

#[pyclass(name = "AddendedOrderedMap")]
pub struct PyAddendedOrderedMap {
    inner: AddendedOrderedMap<u64, Py<PySizedValueBase>, u64>,
}

#[pymethods]
impl PyAddendedOrderedMap {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: AddendedOrderedMap::new(),
        }
    }

    pub fn find<'py>(&self, key: &Bound<'py, PyAny>/*, settings: FindSettings */) -> PyResult<Option<(Py<PyInt>, &Py<PySizedValueBase>)>> {
        let extracted_key: u64 = key.extract()?;

        if let Some((k, v)) = self.inner.find(&extracted_key, FindSettings::new(true)) {
            let k2 = Python::try_attach(|py| {
                k.into_pyobject(py).map(|x| x.unbind())
            }).ok_or_else(|| PyRuntimeError::new_err("Internal conversion error"))??;

            Ok(Some((k2, v)))
        } else {
            Ok(None)
        }
    }
}

