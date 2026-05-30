/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use std::cmp::Ordering;
use std::ops::Deref;
use std::sync::Arc;

use addended_ordered_map::fallible::AddendableKeyFallible;

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::py_alias::PyS;

/*
#[pyclass(
    name = "AddendableKey",
    module = "addended_ordered_map",
    subclass,
    from_py_object
)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub struct PyAddendableKeyBase;

#[pymethods]
impl PyAddendableKeyBase {
    #[new]
    #[pyo3(signature = (*args, **kwargs))]
    pub fn __new__(
        #[expect(unused_variables)] args: &Bound<'_, PyTuple>,
        #[expect(unused_variables)] kwargs: Option<&Bound<'_, PyDict>>,
    ) -> Self {
        Self
    }

    pub fn add_size(&self, _size: PyS) -> PyResult<PyS> {
        Err(PyNotImplementedError::new_err(
            "add_size must be implemented by subclass",
        ))
    }
}
*/

#[derive(Debug)]
#[non_exhaustive]
#[repr(transparent)]
pub struct PyAddendableKeyWrapper(Py<PyAny>);

impl AddendableKeyFallible<PyS, PyErr> for PyAddendableKeyWrapper {
    fn add_size(&self, size: &PyS) -> Result<Self, PyErr> {
        Python::try_attach(|py| self.0.call_method1(py, "__add__", (size,)).map(Self))
            .ok_or_else(|| PyRuntimeError::new_err("Error when adquiring the GIL"))
            .flatten()
    }
}

impl PartialEq for PyAddendableKeyWrapper {
    fn eq(&self, other: &Self) -> bool {
        let eq = Python::try_attach(|py| {
            let x = self.0.call_method1(py, "__eq__", (&other.0,))?;
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
            let x = self.0.call_method1(py, "__lt__", (&other.0,))?;
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
    type Target = Py<PyAny>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
#[repr(transparent)]
pub struct PyAddendableKeyWrapperArc(Arc<PyAddendableKeyWrapper>);

impl PyAddendableKeyWrapperArc {
    pub fn new(value: Py<PyAny>) -> Self {
        Self(Arc::new(PyAddendableKeyWrapper(value)))
    }
}

impl AddendableKeyFallible<PyS, PyErr> for PyAddendableKeyWrapperArc {
    fn add_size(&self, size: &PyS) -> Result<Self, PyErr> {
        Arc::as_ref(&self.0)
            .add_size(size)
            .map(|x| Self(Arc::new(x)))
    }
}

impl PartialEq for PyAddendableKeyWrapperArc {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for PyAddendableKeyWrapperArc {}

impl PartialOrd for PyAddendableKeyWrapperArc {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PyAddendableKeyWrapperArc {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Deref for PyAddendableKeyWrapperArc {
    type Target = Arc<PyAddendableKeyWrapper>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
