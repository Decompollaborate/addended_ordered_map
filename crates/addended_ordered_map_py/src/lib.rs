/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

#![doc = include_str!("../README.md")]
#![warn(clippy::ref_option)]
#![warn(clippy::ref_option_ref)]
#![warn(clippy::useless_let_if_seq)]

use pyo3::prelude::*;

mod py_addended_ordered_map;
pub mod py_alias;
mod py_find_settings;
mod py_into_iter;
mod py_key;
mod py_range_mut;
mod py_sized_value;

pub use py_addended_ordered_map::PyAddendedOrderedMap;
pub use py_find_settings::PyFindSettings;
pub use py_into_iter::PyIntoIter;
pub use py_key::{PyAddendableKeyWrapper, PyAddendableKeyWrapperArc};
pub use py_range_mut::PyRangeMut;
pub use py_sized_value::{PySizedValueBase, PySizedValueBaseWrapper, PySizedValueBaseWrapperArc};

#[pymodule]
fn addended_ordered_map(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyAddendedOrderedMap>()?;
    m.add_class::<PyFindSettings>()?;
    m.add_class::<PySizedValueBase>()?;

    Ok(())
}
