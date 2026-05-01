/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use pyo3::prelude::*;

use addended_ordered_map::FindSettings;

#[pyclass(name = "FindSettings", module = "addended_ordered_map", from_py_object)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PyFindSettings {
    inner: FindSettings,
}

#[pymethods]
impl PyFindSettings {
    #[new]
    pub const fn new(allow_addend: bool) -> Self {
        Self {
            inner: FindSettings::new(allow_addend),
        }
    }
}

impl PyFindSettings {
    pub const fn into_inner(self) -> FindSettings {
        self.inner
    }
}
