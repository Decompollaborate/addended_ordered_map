/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use alloc::sync::Arc;
use alloc::vec;

use pyo3::prelude::*;

use crate::python_bindings::PySizedValueBase;
use crate::AddendedOrderedMap;

#[pyclass(
    name = "AddendedOrderedMapRange",
    module = "addended_ordered_map",
    generic
)]
pub struct PyRangeMut {
    // We can't use real Range/RangeMut because they require a lifetime
    // parameter, which is a no-no for pyo3.
    // Instead we collect the range into a vec iterator.
    inner: vec::IntoIter<(u64, Arc<Py<PySizedValueBase>>)>,
}

#[pymethods]
impl PyRangeMut {
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

impl PyRangeMut {
    pub fn new<SIZE>(
        map: &mut AddendedOrderedMap<u64, Arc<Py<PySizedValueBase>>, SIZE>,
        left: Option<u64>,
        right: Option<u64>,
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
