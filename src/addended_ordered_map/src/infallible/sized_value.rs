/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use core::convert::Infallible;

use crate::fallible::SizedValueFallible;

pub trait SizedValue<SIZE> {
    fn size(&self) -> SIZE;
}

impl<T, S> SizedValueFallible<S, Infallible> for T
where
    T: SizedValue<S>,
{
    fn size(&self) -> Result<S, Infallible> {
        Ok(SizedValue::size(self))
    }
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
