/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(btree_cursors))]
#![warn(clippy::ref_option)]
#![warn(clippy::ref_option_ref)]
#![warn(clippy::useless_let_if_seq)]
// #![warn(clippy::missing_panics_doc)] // TODO

extern crate alloc;

mod addended_ordered_map;
mod find_settings;
mod sized_value;

pub use addended_ordered_map::{AddendedOrderedMap, Range, RangeMut};
pub use find_settings::FindSettings;
pub use sized_value::SizedValue;
