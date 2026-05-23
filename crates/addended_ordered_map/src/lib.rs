/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(btree_cursors))]
#![warn(clippy::ref_option)]
#![warn(clippy::ref_option_ref)]
#![warn(clippy::useless_let_if_seq)]
// #![warn(clippy::missing_panics_doc)] // TODO

extern crate alloc;

pub mod fallible;
mod find_settings;
mod infallible;

pub use find_settings::FindSettings;
pub use infallible::*;
