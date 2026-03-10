/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use pyo3::prelude::*;

mod py_addended_ordered_map;
mod py_sized_value;

pub use py_addended_ordered_map::PyAddendedOrderedMap;
pub use py_sized_value::PySizedValueBase;

#[cfg(feature = "pyo3")]
#[pymodule]
fn spimdisasm(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyAddendedOrderedMap>()?;
    m.add_class::<PySizedValueBase>()?;

    Ok(())
}
