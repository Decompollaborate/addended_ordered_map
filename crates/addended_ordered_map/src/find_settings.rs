/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

// TODO: use the bitflags crate instead of a plain struct
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FindSettings {
    pub(crate) allow_addend: bool,
}

impl FindSettings {
    #[must_use]
    pub const fn new(allow_addend: bool) -> Self {
        Self { allow_addend }
    }

    #[must_use]
    pub const fn allow_addend(&self) -> bool {
        self.allow_addend
    }
}
