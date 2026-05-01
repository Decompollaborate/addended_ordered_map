/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use crate::fallible::SizedValueFallible;

pub trait SizedValue<SIZE> {
    fn size(&self) -> SIZE;
}

impl<T, S, E> SizedValueFallible<S, E> for T
where
    T: SizedValue<S>,
{
    fn size(&self) -> Result<S, E> {
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
