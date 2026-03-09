/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use pyo3::prelude::*;

use crate::{AddendedOrderedMap, FindSettings};

#[pyclass(name = "AddendedOrderedMap")]
struct PyAddendedOrderedMap {
    inner: AddendedOrderedMap<u64, Py<PyAny>, Option<u64>>,
}

#[pymethods]
impl PyAddendedOrderedMap {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: AddendedOrderedMap::new(),
        }
    }

    /*
    pub fn find<'py>(&self, key: &Bound<'py, PyAny>/*, settings: FindSettings */) -> PyResult<Option<(Py<PyAny>, &Py<PyAny>)>> {
        let extracted_key = key.extract()?;
        self.inner.find(extracted_key, FindSettings::new(true));
        todo!()
    }
    */
}
