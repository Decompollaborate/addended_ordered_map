/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use pyo3::exceptions::PyNotImplementedError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};

use crate::SizedValue;

#[pyclass(name = "SizedValue", module="addended_ordered_map", subclass, extends=pyo3::types::PyAny)]
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

    pub fn get_size(&self) -> PyResult<u64> {
        Err(PyNotImplementedError::new_err(
            "get_size must be implemented by subclass",
        ))
    }
}

impl SizedValue<u64> for Py<PySizedValueBase> {
    fn size(&self) -> u64 {
        Python::try_attach(|py| {
            let size = self.call_method0(py, "get_size").unwrap();
            size.extract(py).unwrap()
        })
        .unwrap()
    }
}
