/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use alloc::collections::btree_map;
use alloc::sync::Arc;

use pyo3::prelude::*;

use crate::python_bindings::PySizedValueBase;
use crate::AddendedOrderedMap;

#[pyclass(
    name = "AddendedOrderedMapIter",
    module = "addended_ordered_map",
    generic
)]
#[must_use]
pub struct PyIntoIter {
    inner: btree_map::IntoIter<u64, Arc<Py<PySizedValueBase>>>,
}

#[pymethods]
impl PyIntoIter {
    pub fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    pub fn __next__(mut slf: PyRefMut<Self>) -> Option<(u64, Py<PySizedValueBase>)> {
        slf.inner.next().map(|(k, v)| {
            let new_v = Python::try_attach(|py| v.clone_ref(py)).unwrap();
            (k, new_v)
        })
    }
}

impl PyIntoIter {
    pub fn new(map: AddendedOrderedMap<u64, Arc<Py<PySizedValueBase>>, u64>) -> Self {
        Self {
            inner: map.into_iter(),
        }
    }
}
