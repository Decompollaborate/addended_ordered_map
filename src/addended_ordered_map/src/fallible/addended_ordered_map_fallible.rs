/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use alloc::collections::btree_map::{self, BTreeMap};
use core::{
    fmt,
    marker::PhantomData,
    ops::{Add, RangeBounds},
};

#[cfg(not(feature = "nightly"))]
use ::polonius_the_crab::prelude::*;

#[cfg(feature = "nightly")]
use core::ops::Bound;

use super::SizedValueFallible;
use crate::FindSettings;

pub type Range<'a, K, V> = btree_map::Range<'a, K, V>;
pub type RangeMut<'a, K, V> = btree_map::RangeMut<'a, K, V>;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AddendedOrderedMapFallible<K, V, SIZE>
where
    K: Ord + Copy + Add<SIZE, Output = K>,
    V: SizedValueFallible<SIZE>,
{
    inner: BTreeMap<K, V>,
    phantom: PhantomData<SIZE>,
}

impl<K, V, SIZE> AddendedOrderedMapFallible<K, V, SIZE>
where
    K: Ord + Copy + Add<SIZE, Output = K>,
    V: SizedValueFallible<SIZE>,
{
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
            phantom: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<K, V, SIZE> AddendedOrderedMapFallible<K, V, SIZE>
where
    K: Ord + Copy + Add<SIZE, Output = K>,
    V: SizedValueFallible<SIZE>,
{
    #[must_use]
    pub fn find(&self, key: &K, settings: FindSettings) -> Result<Option<(K, &V)>, V::E> {
        if !settings.allow_addend {
            Ok(self.inner.get(key).map(|v| (*key, v)))
        } else {
            let mut range = self.inner.range(..=key);

            let Some((other_key, v)) = range.next_back() else {
                return Ok(None);
            };

            let other_key = *other_key;
            Ok(if &other_key == key || *key < other_key + v.size()? {
                Some((other_key, v))
            } else {
                None
            })
        }
    }

    #[must_use]
    pub fn find_key(&self, key: &K, settings: FindSettings) -> Result<Option<K>, V::E> {
        self.find(key, settings).map(|x| x.map(|y| y.0))
    }

    #[must_use]
    pub fn find_value(&self, key: &K, settings: FindSettings) -> Result<Option<&V>, V::E> {
        self.find(key, settings).map(|x| x.map(|y| y.1))
    }

    #[must_use]
    pub fn find_mut(
        &mut self,
        key: &K,
        settings: FindSettings,
    ) -> Result<Option<(K, &mut V)>, V::E> {
        if !settings.allow_addend {
            Ok(self.inner.get_mut(key).map(|v| (*key, v)))
        } else {
            let mut range = self.inner.range_mut(..=key);

            let Some((other_key, v)) = range.next_back() else {
                return Ok(None);
            };

            let other_key = *other_key;
            Ok(if &other_key == key || *key < other_key + v.size()? {
                Some((other_key, v))
            } else {
                None
            })
        }
    }
}

#[cfg(not(feature = "nightly"))]
fn add_impl<'slf, K, V, SIZE, F>(
    mut slf: &'slf mut AddendedOrderedMapFallible<K, V, SIZE>,
    key: &K,
    settings: FindSettings,
    default: F,
) -> Result<(&'slf mut V, bool), V::E>
where
    K: Ord + Copy + Add<SIZE, Output = K>,
    V: SizedValueFallible<SIZE>,
    F: FnOnce() -> Result<(K, V), V::E>,
{
    // TODO: get rid of the polonius stuff when the new borrow checker has been released.

    polonius!(|slf| -> Result<(&'polonius mut V, bool), V::E> {
        let ret = match slf.find_mut(key, settings) {
            Ok(r) => r,
            Err(e) => {
                polonius_return!(Err(e))
            }
        };
        if let Some((_k, v)) = ret {
            polonius_return!(Ok((v, false)));
        }
    });

    let (k, v) = default()?;
    let entry = slf.inner.entry(k);

    let newly_created = matches!(entry, btree_map::Entry::Vacant(_));
    Ok((entry.or_insert(v), newly_created))
}

#[cfg(feature = "nightly")]
fn add_impl<'slf, K, V, SIZE, F>(
    slf: &'slf mut AddendedOrderedMapFallible<K, V, SIZE>,
    key: &K,
    settings: FindSettings,
    default: F,
) -> Result<(&'slf mut V, bool), V::E>
where
    K: Ord + Copy + Add<SIZE, Output = K>,
    V: SizedValueFallible<SIZE>,
    F: FnOnce() -> Result<(K, V), V::E>,
{
    let mut cursor = slf.inner.upper_bound_mut(Bound::Included(key));

    let must_insert_new = if let Some((other_key, v)) = cursor.peek_prev() {
        if key == other_key {
            false
        } else if !settings.allow_addend {
            true
        } else {
            *key >= *other_key + v.size()?
        }
    } else {
        true
    };

    if must_insert_new {
        let (k, v) = default()?;
        cursor
            .insert_before(k, v)
            .expect("This should not be able to panic");
    }

    //let sym = unsafe { &mut *(cursor.peek_prev().unwrap().1 as *mut SymbolMetadata) };
    Ok((into_prev_and_next(cursor).0.unwrap().1, must_insert_new))
}

#[cfg(feature = "nightly")]
fn into_prev_and_next<'a, K, V>(
    mut cursor: btree_map::CursorMut<'a, K, V>,
) -> (Option<(&'a K, &'a mut V)>, Option<(&'a K, &'a mut V)>) {
    let prev: Option<(&'a K, &'a mut V)> = cursor.peek_prev().map(|(k, v)| {
        let ptr_k = k as *const K;
        let ptr_v = v as *mut V;
        unsafe { (&*ptr_k, &mut *ptr_v) }
    });
    let next: Option<(&'a K, &'a mut V)> = cursor.peek_next().map(|(k, v)| {
        let ptr_k = k as *const K;
        let ptr_v = v as *mut V;
        unsafe { (&*ptr_k, &mut *ptr_v) }
    });

    (prev, next)
}

impl<K, V, SIZE> AddendedOrderedMapFallible<K, V, SIZE>
where
    K: Ord + Copy + Add<SIZE, Output = K>,
    V: SizedValueFallible<SIZE>,
{
    pub fn find_mut_or_insert_with<F>(
        &mut self,
        key: K,
        settings: FindSettings,
        default: F,
    ) -> Result<(&mut V, bool), V::E>
    where
        K: Copy,
        F: FnOnce() -> Result<V, V::E>,
    {
        add_impl(self, &key, settings, || Ok((key, default()?)))
    }

    pub fn find_mut_or_insert_with_key_value<F>(
        &mut self,
        key: &K,
        settings: FindSettings,
        default: F,
    ) -> Result<(&mut V, bool), V::E>
    where
        F: FnOnce() -> Result<(K, V), V::E>,
    {
        add_impl(self, key, settings, default)
    }
}

impl<K, V, SIZE> AddendedOrderedMapFallible<K, V, SIZE>
where
    K: Ord + Copy + Add<SIZE, Output = K>,
    V: SizedValueFallible<SIZE>,
{
    pub fn contains_key_exact(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }

    pub fn pop_exact(&mut self, key: &K) -> Option<(K, V)> {
        self.inner.remove_entry(key)
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.inner.retain(f);
    }
}

impl<K, V, SIZE> AddendedOrderedMapFallible<K, V, SIZE>
where
    K: Ord + Copy + Add<SIZE, Output = K>,
    V: SizedValueFallible<SIZE>,
{
    pub fn iter(&self) -> btree_map::Iter<'_, K, V> {
        self.inner.iter()
    }

    /*
    pub fn iter_mut(&mut self) -> btree_map::IterMut<K, V> {
        self.inner.iter_mut()
    }
    */

    pub fn range<R>(&self, range: R) -> Range<'_, K, V>
    where
        R: RangeBounds<K>,
    {
        self.inner.range(range)
    }

    pub fn range_mut<R>(&mut self, range: R) -> RangeMut<'_, K, V>
    where
        R: RangeBounds<K>,
    {
        self.inner.range_mut(range)
    }

    #[cfg(feature = "extract_if")]
    pub fn extract_if<F, R>(&mut self, range: R, pred: F) -> btree_map::ExtractIf<'_, K, V, R, F>
    where
        R: RangeBounds<K>,
        F: FnMut(&K, &mut V) -> bool,
    {
        self.inner.extract_if(range, pred)
    }

    pub fn keys(&self) -> btree_map::Keys<'_, K, V> {
        self.inner.keys()
    }

    pub fn values(&self) -> btree_map::Values<'_, K, V> {
        self.inner.values()
    }

    pub fn values_mut(&mut self) -> btree_map::ValuesMut<'_, K, V> {
        self.inner.values_mut()
    }

    pub fn into_keys(self) -> btree_map::IntoKeys<K, V> {
        self.inner.into_keys()
    }

    pub fn into_values(self) -> btree_map::IntoValues<K, V> {
        self.inner.into_values()
    }
}

impl<K, V, SIZE> fmt::Debug for AddendedOrderedMapFallible<K, V, SIZE>
where
    K: fmt::Debug + Ord + Copy + Add<SIZE, Output = K>,
    V: fmt::Debug + SizedValueFallible<SIZE>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Manually implement Debug to hide the `inner` indirection
        write!(f, "AddendedOrderedMapFallible {:?}", self.inner)
    }
}

impl<K, V, SIZE> Default for AddendedOrderedMapFallible<K, V, SIZE>
where
    K: Ord + Copy + Add<SIZE, Output = K>,
    V: SizedValueFallible<SIZE>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, K, V, SIZE> IntoIterator for &'a AddendedOrderedMapFallible<K, V, SIZE>
where
    K: Ord + Copy + Add<SIZE, Output = K>,
    V: SizedValueFallible<SIZE>,
{
    type Item = (&'a K, &'a V);
    type IntoIter = btree_map::Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/*
impl<'a, K, V, SIZE> IntoIterator for &'a mut AddendedOrderedMapFallible<K, V, SIZE>
{
    type Item = (&'a K, &'a mut V);
    type IntoIter = btree_map::IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
*/

impl<K, V, SIZE> IntoIterator for AddendedOrderedMapFallible<K, V, SIZE>
where
    K: Ord + Copy + Add<SIZE, Output = K>,
    V: SizedValueFallible<SIZE>,
{
    type Item = (K, V);
    type IntoIter = btree_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl SizedValueFallible<u32> for Option<u32> {
        type E = core::convert::Infallible;

        fn size(&self) -> Result<u32, Self::E> {
            Ok(self.unwrap_or(1))
        }
    }

    #[test]
    fn check_bounds() {
        let mut map: AddendedOrderedMapFallible<u32, Option<u32>, u32> =
            AddendedOrderedMapFallible::new();

        map.find_mut_or_insert_with(0x100C, FindSettings::new(true), || Ok(None))
            .unwrap();
        map.find_mut_or_insert_with(0x1000, FindSettings::new(true), || Ok(Some(4)))
            .unwrap();
        map.find_mut_or_insert_with(0x1004, FindSettings::new(true), || Ok(Some(4)))
            .unwrap();

        assert_eq!(
            Ok(Some((0x1000, &Some(4)))),
            map.find(&0x1000, FindSettings::new(true)),
        );

        assert_eq!(
            Ok(Some((0x1000, &Some(4)))),
            map.find(&0x1002, FindSettings::new(true)),
        );

        assert_eq!(Ok(None), map.find(&0x0F00, FindSettings::new(true)),);

        assert_eq!(Ok(None), map.find(&0x2000, FindSettings::new(true)),);

        assert_eq!(Ok(None), map.find(&0x1002, FindSettings::new(false)),);

        assert_eq!(Ok(None), map.find(&0x1008, FindSettings::new(true)),);
    }
}
