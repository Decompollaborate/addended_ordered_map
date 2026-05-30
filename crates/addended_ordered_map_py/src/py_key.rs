/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use std::cmp::Ordering;
use std::ops::Deref;
use std::sync::Arc;

use addended_ordered_map::fallible::AddendableKeyFallible;

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::py_alias::PyS;

#[derive(Clone, Debug)]
#[non_exhaustive]
#[repr(transparent)]
pub struct PyAddendableKeyWrapper(Arc<Py<PyAny>>);

impl PyAddendableKeyWrapper {
    pub fn new(value: Py<PyAny>) -> Self {
        Self(Arc::new(value))
    }
}

impl AddendableKeyFallible<PyS, PyErr> for PyAddendableKeyWrapper {
    fn add_size(&self, size: &PyS) -> Result<Self, PyErr> {
        Python::try_attach(|py| self.0.call_method1(py, "__add__", (size,)).map(Self::new))
            .ok_or_else(|| PyRuntimeError::new_err("Error when adquiring the GIL"))
            .flatten()
    }
}

impl PartialEq for PyAddendableKeyWrapper {
    fn eq(&self, other: &Self) -> bool {
        let eq = Python::try_attach(|py| {
            let x = self.0.call_method1(py, "__eq__", (other.0.deref(),))?;
            let y: bool = x.extract(py)?;
            Ok(y)
        })
        .ok_or_else(|| PyRuntimeError::new_err("Error when adquiring the GIL"))
        .flatten();

        matches!(eq, Ok(true))
    }
}

impl Eq for PyAddendableKeyWrapper {}

impl PartialOrd for PyAddendableKeyWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PyAddendableKeyWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        let lt = Python::try_attach(|py| {
            let x = self.0.call_method1(py, "__lt__", (other.0.deref(),))?;
            let y: bool = x.extract(py)?;
            Ok(y)
        })
        .ok_or_else(|| PyRuntimeError::new_err("Error when adquiring the GIL"))
        .flatten();

        if matches!(lt, Ok(true)) {
            Ordering::Less
        } else if self.eq(other) {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

impl Deref for PyAddendableKeyWrapper {
    type Target = Arc<Py<PyAny>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
