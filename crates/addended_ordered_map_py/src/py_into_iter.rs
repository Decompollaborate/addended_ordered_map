/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use std::collections::btree_map;

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use addended_ordered_map::fallible::AddendedOrderedMapFallible;

use crate::py_alias::{PyK, PyKW, PyS, PyV, PyVW};

#[pyclass(
    name = "AddendedOrderedMapIter",
    module = "addended_ordered_map",
    generic
)]
#[must_use]
pub struct PyIntoIter {
    inner: btree_map::IntoIter<PyKW, PyVW>,
}

#[pymethods]
impl PyIntoIter {
    pub fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    pub fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<(PyK, PyV)>> {
        slf.inner
            .next()
            .map(|(k, v)| {
                let (new_k, new_v) = Python::try_attach(|py| (k.clone_ref(py), v.clone_ref(py)))
                    .ok_or_else(|| PyRuntimeError::new_err("Internal error"))?;
                Ok((new_k, new_v))
            })
            .transpose()
    }
}

impl PyIntoIter {
    pub fn new(map: AddendedOrderedMapFallible<PyKW, PyVW, PyS, PyErr>) -> Self {
        Self {
            inner: map.into_iter(),
        }
    }
}
