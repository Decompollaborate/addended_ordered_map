/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use pyo3::prelude::*;

use crate::{PyAddendableKeyWrapper, PySizedValueBase, PySizedValueBaseWrapperArc};

pub type PyK = Py<PyAny>;
pub type PyKW = PyAddendableKeyWrapper;
pub type PyV = Py<PySizedValueBase>;
pub type PyVW = PySizedValueBaseWrapperArc;
pub type PyS = Py<PyAny>;
