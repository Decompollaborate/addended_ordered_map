/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

pub trait AddendableKeyFallible<SIZE, E>
where
    Self: Sized,
{
    fn add_size(&self, size: &SIZE) -> Result<Self, E>;
}
