/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyInt;

use crate::{AddendedOrderedMap, FindSettings};
use crate::python_bindings::PySizedValueBase;

#[pyclass(name = "AddendedOrderedMap", module="addended_ordered_map", generic)]
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
        let find_settings = FindSettings::new(true);

        if let Some((k, v)) = self.inner.find(&extracted_key, find_settings) {
            let k2 = Python::try_attach(|py| {
                k.into_pyobject(py).map(|x| x.unbind())
            }).ok_or_else(|| PyRuntimeError::new_err("Internal conversion error"))??;

            Ok(Some((k2, v)))
        } else {
            Ok(None)
        }
    }

    pub fn find_or_insert<'py>(&mut self, key: &Bound<'py, PyAny>, new_value: Py<PySizedValueBase>) -> PyResult<(&Py<PySizedValueBase>, bool)> {
        let extracted_key: u64 = key.extract()?;
        let find_settings = FindSettings::new(true);

        let (v, newly_created) = self.inner.find_mut_or_insert_with(extracted_key, find_settings, || new_value);
        Ok((v, newly_created))
    }

    pub fn find_or_insert_with<'py>(&mut self, key: &Bound<'py, PyAny>, new_default: &Bound<'py, PyAny>) -> PyResult<(&Py<PySizedValueBase>, bool)> {
        // if !new_default.is_callable()
        let extracted_key: u64 = key.extract()?;
        let find_settings = FindSettings::new(true);

        let (v, newly_created) = self.inner.find_mut_or_insert_with(extracted_key, find_settings, || {
            // call a callable python object/function/lambda/etc
            let result = new_default.call0().unwrap();
            let casted = result.cast().map(|x| x.clone().unbind());
            casted.unwrap()
        });
        Ok((v, newly_created))
    }
}
