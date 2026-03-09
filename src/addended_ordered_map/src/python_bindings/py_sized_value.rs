/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use pyo3::prelude::*;

use crate::{SizedValue};

#[pyclass(name = "SizedValue", subclass)]
struct PySizedValueBase {}

impl PySizedValueBase {
    fn get_size_from_py_subclass<T>(&self) -> T {
        
        todo!()
    }
}


