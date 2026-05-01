/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use core::ops::Add;

use crate::fallible::AddendableKeyFallible;

pub trait AddendableKey<SIZE>
where
    Self: Sized,
{
    fn add_size(&self, size: &SIZE) -> Self;
}

impl<T, S, E> AddendableKeyFallible<S, E> for T
where
    T: AddendableKey<S>,
{
    fn add_size(&self, size: &S) -> Result<Self, E> {
        Ok(AddendableKey::add_size(self, size))
    }
}

impl<S> AddendableKey<S> for S
where
    S: Copy + Add<S, Output = S>,
{
    fn add_size(&self, size: &S) -> Self {
        *self + *size
    }
}
