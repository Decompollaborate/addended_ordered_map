/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

pub trait SizedValue<SIZE> {
    fn size(&self) -> SIZE;
}

// TODO: consider removing?
impl<T> SizedValue<T> for T
where
    T: Copy,
{
    fn size(&self) -> T {
        *self
    }
}
