/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use core::convert::Infallible;

pub trait SizedValueFallible<SIZE> {
    type E;

    fn size(&self) -> Result<SIZE, Self::E>;
}

// TODO: consider removing?
impl<T> SizedValueFallible<T> for T
where
    T: Copy,
{
    type E = Infallible;

    fn size(&self) -> Result<T, Self::E> {
        Ok(*self)
    }
}
