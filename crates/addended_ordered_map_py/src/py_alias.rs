/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use pyo3::prelude::*;

use crate::{
    PyAddendableKeyWrapper, PyAddendableKeyWrapperArc, PySizedValueBase, PySizedValueBaseWrapper,
    PySizedValueBaseWrapperArc,
};

pub type PyK = Py<PyAny>;
pub type PyKW = PyAddendableKeyWrapper;
pub type PyKWA = PyAddendableKeyWrapperArc;
pub type PyV = Py<PySizedValueBase>;
pub type PyVW = PySizedValueBaseWrapper;
pub type PyVWA = PySizedValueBaseWrapperArc;
pub type PyS = Py<PyAny>;
