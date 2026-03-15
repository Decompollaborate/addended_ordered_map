/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use core::convert::Infallible;

use crate::fallible::SizedValueFallible;

pub trait SizedValue<SIZE> {
    fn size(&self) -> SIZE;
}

// TODO: consider removing?
impl<S> SizedValue<S> for S
where
    S: Copy,
{
    fn size(&self) -> S {
        *self
    }
}

impl<T, S> SizedValueFallible<S> for T
where
    T: SizedValue<S>,
{
    type E = Infallible;

    fn size(&self) -> Result<S, Self::E> {
        Ok(SizedValue::size(self))
    }
}
