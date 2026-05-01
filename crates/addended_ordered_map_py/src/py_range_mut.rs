/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use std::vec;

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use addended_ordered_map::fallible::AddendedOrderedMapFallible;

use crate::py_alias::{PyK, PyS, PyV, PyVWA};

#[pyclass(
    name = "AddendedOrderedMapRange",
    module = "addended_ordered_map",
    generic
)]
#[must_use]
pub struct PyRangeMut {
    // We can't use real Range/RangeMut because they require a lifetime
    // parameter, which is a no-no for pyo3.
    // Instead we collect the range into a vec iterator.
    inner: vec::IntoIter<(PyK, PyVWA)>,
}

#[pymethods]
impl PyRangeMut {
    pub fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    pub fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<(PyK, PyV)>> {
        slf.inner
            .next()
            .map(|(k, v)| {
                let new_v = Python::try_attach(|py| v.clone_ref(py))
                    .ok_or_else(|| PyRuntimeError::new_err("Internal error"))?;
                Ok((k, new_v))
            })
            .transpose()
    }
}

impl PyRangeMut {
    pub fn new(
        map: &mut AddendedOrderedMapFallible<PyK, PyVWA, PyS, PyErr>,
        left: Option<PyK>,
        right: Option<PyK>,
    ) -> Self {
        let range = match (left, right) {
            (Some(l), Some(r)) => map.range(l..r),
            (Some(l), None) => map.range(l..),
            (None, Some(r)) => map.range(..r),
            (None, None) => map.range(..),
        };
        // let inner: vec::Vec<_> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        let inner: vec::Vec<_> = range.map(|(k, v)| (k.clone(), v.clone())).collect();
        Self {
            inner: inner.into_iter(),
        }
    }
}
