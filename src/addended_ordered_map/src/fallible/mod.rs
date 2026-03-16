/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

mod addendable_key_fallible;
mod addended_ordered_map_fallible;
mod sized_value_fallible;

pub use addendable_key_fallible::AddendableKeyFallible;
pub use addended_ordered_map_fallible::{AddendedOrderedMapFallible, Range, RangeMut};
pub use sized_value_fallible::SizedValueFallible;
