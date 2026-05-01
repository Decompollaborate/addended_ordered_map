/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use pyo3::prelude::*;

use crate::python_bindings::PySizedValueBase;

pub type PyK = u64;
pub type PyV = Py<PySizedValueBase>;
pub type PyS = u64;
