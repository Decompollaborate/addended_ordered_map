/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use pyo3::prelude::*;

use crate::fallible::AddendableKeyFallible;
use crate::python_bindings::PySizedValueBase;

pub type PyK = u64;
pub type PyV = Py<PySizedValueBase>;
pub type PyS = u64;

impl AddendableKeyFallible<u64, PyErr> for u64 {
    fn add_size(&self, size: &u64) -> Result<Self, PyErr> {
        Ok(*self + *size)
    }
}
