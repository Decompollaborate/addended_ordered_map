/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

/// A value that can report its size through a fallible operation.
///
/// This trait is used by fallible maps to determine how far the key range
/// extends. If the size cannot be produced, the error is propagated to the
/// caller.
pub trait SizedValueFallible<SIZE, E> {
    fn size(&self) -> Result<SIZE, E>;
}
