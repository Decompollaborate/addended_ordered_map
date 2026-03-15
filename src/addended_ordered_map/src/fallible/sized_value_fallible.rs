/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

pub trait SizedValueFallible<SIZE> {
    type E;

    fn size(&self) -> Result<SIZE, Self::E>;
}
