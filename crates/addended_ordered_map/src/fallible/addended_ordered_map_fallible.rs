/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use alloc::collections::btree_map::{self, BTreeMap};
use core::{
    fmt,
    marker::PhantomData,
    ops::{Bound, RangeBounds},
};

#[cfg(not(feature = "nightly"))]
use ::polonius_the_crab::prelude::*;

#[cfg(feature = "nightly")]
use core::ops::Bound;

use super::{AddendableKeyFallible, SizedValueFallible};
use crate::FindSettings;

pub type Range<'a, K, V> = btree_map::Range<'a, K, V>;
pub type RangeMut<'a, K, V> = btree_map::RangeMut<'a, K, V>;

pub struct AddendedOrderedMapFallible<K, V, SIZE, E>
where
    K: Ord + AddendableKeyFallible<SIZE, E>,
    V: SizedValueFallible<SIZE, E>,
{
    inner: BTreeMap<K, V>,
    phantom: PhantomData<(SIZE, E)>,
}

impl<K, V, SIZE, E> AddendedOrderedMapFallible<K, V, SIZE, E>
where
    K: Ord + AddendableKeyFallible<SIZE, E>,
    V: SizedValueFallible<SIZE, E>,
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

impl<K, V, SIZE, E> AddendedOrderedMapFallible<K, V, SIZE, E>
where
    K: Ord + AddendableKeyFallible<SIZE, E>,
    V: SizedValueFallible<SIZE, E>,
{
    #[must_use = "This is a lookup function, there are no side-effects on the mapping."]
    pub fn find(&self, key: &K, settings: FindSettings) -> Result<Option<(&K, &V)>, E> {
        let mut range = self.inner.range(..=key);

        let ret = if let Some((other_key, v)) = range.next_back() {
            if other_key == key
                || (settings.allow_addend && key < &other_key.add_size(&v.size()?)?)
            {
                Some((other_key, v))
            } else {
                None
            }
        } else {
            None
        };

        Ok(ret)
    }

    #[must_use = "This is a lookup function, there are no side-effects on the mapping."]
    pub fn find_key(&self, key: &K, settings: FindSettings) -> Result<Option<&K>, E> {
        self.find(key, settings).map(|x| x.map(|y| y.0))
    }

    #[must_use = "This is a lookup function, there are no side-effects on the mapping."]
    pub fn find_value(&self, key: &K, settings: FindSettings) -> Result<Option<&V>, E> {
        self.find(key, settings).map(|x| x.map(|y| y.1))
    }

    #[must_use = "This is a lookup function, there are no side-effects on the mapping."]
    pub fn find_mut(&mut self, key: &K, settings: FindSettings) -> Result<Option<(&K, &mut V)>, E> {
        let mut range = self.inner.range_mut(..=key);

        let ret = if let Some((other_key, v)) = range.next_back() {
            if other_key == key
                || (settings.allow_addend && key < &other_key.add_size(&v.size()?)?)
            {
                Some((other_key, v))
            } else {
                None
            }
        } else {
            None
        };

        Ok(ret)
    }

    #[must_use = "This is a lookup function, there are no side-effects on the mapping."]
    pub fn find_left_of(&self, key: &K, inclusive: bool) -> Option<(&K, &V)> {
        let start = Bound::Unbounded;
        let end = if inclusive {
            Bound::Included(key)
        } else {
            Bound::Excluded(key)
        };

        self.inner.range((start, end)).next_back()
    }

    #[must_use = "This is a lookup function, there are no side-effects on the mapping."]
    pub fn find_right_of(&self, key: &K, inclusive: bool) -> Option<(&K, &V)> {
        let start = if inclusive {
            Bound::Included(key)
        } else {
            Bound::Excluded(key)
        };
        let end = Bound::Unbounded;

        self.inner.range((start, end)).next()
    }

    #[must_use = "This is a lookup function, there are no side-effects on the mapping."]
    pub fn find_left_of_mut(&mut self, key: &K, inclusive: bool) -> Option<(&K, &mut V)> {
        let start = Bound::Unbounded;
        let end = if inclusive {
            Bound::Included(key)
        } else {
            Bound::Excluded(key)
        };

        self.inner.range_mut((start, end)).next_back()
    }

    #[must_use = "This is a lookup function, there are no side-effects on the mapping."]
    pub fn find_right_of_mut(&mut self, key: &K, inclusive: bool) -> Option<(&K, &mut V)> {
        let start = if inclusive {
            Bound::Included(key)
        } else {
            Bound::Excluded(key)
        };
        let end = Bound::Unbounded;

        self.inner.range_mut((start, end)).next()
    }
}

#[cfg(not(feature = "nightly"))]
fn add_impl<'slf, K, V, SIZE, E, F>(
    mut slf: &'slf mut AddendedOrderedMapFallible<K, V, SIZE, E>,
    key: &K,
    settings: FindSettings,
    default: F,
) -> Result<(&'slf mut V, bool), E>
where
    K: Ord + AddendableKeyFallible<SIZE, E>,
    V: SizedValueFallible<SIZE, E>,
    F: FnOnce() -> Result<(K, V), E>,
{
    // TODO: get rid of the polonius stuff when the new borrow checker has been released.

    polonius!(|slf| -> Result<(&'polonius mut V, bool), E> {
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
fn add_impl<'slf, K, V, SIZE, E, F>(
    slf: &'slf mut AddendedOrderedMapFallible<K, V, SIZE, E>,
    key: &K,
    settings: FindSettings,
    default: F,
) -> Result<(&'slf mut V, bool), E>
where
    K: Ord + AddendableKeyFallible<SIZE, E>,
    V: SizedValueFallible<SIZE, E>,
    F: FnOnce() -> Result<(K, V), E>,
{
    let mut cursor = slf.inner.upper_bound_mut(Bound::Included(key));

    let must_insert_new = if let Some((other_key, v)) = cursor.peek_prev() {
        if key == other_key {
            false
        } else if !settings.allow_addend {
            true
        } else {
            key >= &other_key.add_size(&v.size()?)?
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

impl<K, V, SIZE, E> AddendedOrderedMapFallible<K, V, SIZE, E>
where
    K: Ord + AddendableKeyFallible<SIZE, E>,
    V: SizedValueFallible<SIZE, E>,
{
    pub fn find_mut_or_insert_with<F>(
        &mut self,
        key: K,
        settings: FindSettings,
        default: F,
    ) -> Result<(&mut V, bool), E>
    where
        K: Copy,
        F: FnOnce() -> Result<V, E>,
    {
        add_impl(self, &key, settings, || Ok((key, default()?)))
    }

    pub fn find_mut_or_insert_with_key_value<F>(
        &mut self,
        key: &K,
        settings: FindSettings,
        default: F,
    ) -> Result<(&mut V, bool), E>
    where
        F: FnOnce() -> Result<(K, V), E>,
    {
        add_impl(self, key, settings, default)
    }
}

impl<K, V, SIZE, E> AddendedOrderedMapFallible<K, V, SIZE, E>
where
    K: Ord + AddendableKeyFallible<SIZE, E>,
    V: SizedValueFallible<SIZE, E>,
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

impl<K, V, SIZE, E> AddendedOrderedMapFallible<K, V, SIZE, E>
where
    K: Ord + AddendableKeyFallible<SIZE, E>,
    V: SizedValueFallible<SIZE, E>,
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

impl<K, V, SIZE, E> fmt::Debug for AddendedOrderedMapFallible<K, V, SIZE, E>
where
    K: fmt::Debug + Ord + AddendableKeyFallible<SIZE, E>,
    V: fmt::Debug + SizedValueFallible<SIZE, E>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Manually implement Debug to hide the `inner` indirection
        write!(f, "AddendedOrderedMapFallible {:?}", self.inner)
    }
}

impl<K, V, SIZE, E> Clone for AddendedOrderedMapFallible<K, V, SIZE, E>
where
    K: Clone + Ord + AddendableKeyFallible<SIZE, E>,
    V: Clone + SizedValueFallible<SIZE, E>,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            phantom: self.phantom,
        }
    }
}

impl<K, V, SIZE, E> core::hash::Hash for AddendedOrderedMapFallible<K, V, SIZE, E>
where
    K: core::hash::Hash + Ord + AddendableKeyFallible<SIZE, E>,
    V: core::hash::Hash + SizedValueFallible<SIZE, E>,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
        self.phantom.hash(state);
    }
}

impl<K, V, SIZE, E> PartialEq for AddendedOrderedMapFallible<K, V, SIZE, E>
where
    K: PartialEq + Ord + AddendableKeyFallible<SIZE, E>,
    V: PartialEq + SizedValueFallible<SIZE, E>,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner && self.phantom == other.phantom
    }
}

impl<K, V, SIZE, E> Eq for AddendedOrderedMapFallible<K, V, SIZE, E>
where
    K: Eq + Ord + AddendableKeyFallible<SIZE, E>,
    V: Eq + SizedValueFallible<SIZE, E>,
{
}

impl<K, V, SIZE, E> PartialOrd for AddendedOrderedMapFallible<K, V, SIZE, E>
where
    K: PartialOrd + Ord + AddendableKeyFallible<SIZE, E>,
    V: PartialOrd + SizedValueFallible<SIZE, E>,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match self.inner.partial_cmp(&other.inner) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.phantom.partial_cmp(&other.phantom)
    }
}

impl<K, V, SIZE, E> Ord for AddendedOrderedMapFallible<K, V, SIZE, E>
where
    K: Ord + AddendableKeyFallible<SIZE, E>,
    V: Ord + SizedValueFallible<SIZE, E>,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.inner
            .cmp(&other.inner)
            .then_with(|| self.phantom.cmp(&other.phantom))
    }
}

impl<K, V, SIZE, E> Default for AddendedOrderedMapFallible<K, V, SIZE, E>
where
    K: Ord + AddendableKeyFallible<SIZE, E>,
    V: SizedValueFallible<SIZE, E>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, K, V, SIZE, E> IntoIterator for &'a AddendedOrderedMapFallible<K, V, SIZE, E>
where
    K: Ord + AddendableKeyFallible<SIZE, E>,
    V: SizedValueFallible<SIZE, E>,
{
    type Item = (&'a K, &'a V);
    type IntoIter = btree_map::Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/*
impl<'a, K, V, SIZE> IntoIterator for &'a mut AddendedOrderedMapFallible<K, V, SIZE, E>
{
    type Item = (&'a K, &'a mut V);
    type IntoIter = btree_map::IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
*/

impl<K, V, SIZE, E> IntoIterator for AddendedOrderedMapFallible<K, V, SIZE, E>
where
    K: Ord + AddendableKeyFallible<SIZE, E>,
    V: SizedValueFallible<SIZE, E>,
{
    type Item = (K, V);
    type IntoIter = btree_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use core::convert::Infallible;

    use super::*;

    #[test]
    fn check_bounds() {
        let mut map: AddendedOrderedMapFallible<u32, Option<u32>, u32, Infallible> =
            AddendedOrderedMapFallible::new();

        map.find_mut_or_insert_with(0x100C, FindSettings::new(true), || Ok(None))
            .unwrap();
        map.find_mut_or_insert_with(0x1000, FindSettings::new(true), || Ok(Some(4)))
            .unwrap();
        map.find_mut_or_insert_with(0x1004, FindSettings::new(true), || Ok(Some(4)))
            .unwrap();

        assert_eq!(
            Ok(Some((&0x1000, &Some(4)))),
            map.find(&0x1000, FindSettings::new(true)),
        );

        assert_eq!(
            Ok(Some((&0x1000, &Some(4)))),
            map.find(&0x1002, FindSettings::new(true)),
        );

        assert_eq!(Ok(None), map.find(&0x0F00, FindSettings::new(true)),);

        assert_eq!(Ok(None), map.find(&0x2000, FindSettings::new(true)),);

        assert_eq!(Ok(None), map.find(&0x1002, FindSettings::new(false)),);

        assert_eq!(Ok(None), map.find(&0x1008, FindSettings::new(true)),);
    }

    #[test]
    fn check_left_right() {
        let mut map: AddendedOrderedMapFallible<u32, Option<u32>, u32, Infallible> =
            AddendedOrderedMapFallible::new();

        map.find_mut_or_insert_with(0x100C, FindSettings::new(true), || Ok(None))
            .unwrap();
        map.find_mut_or_insert_with(0x1000, FindSettings::new(true), || Ok(Some(4)))
            .unwrap();
        map.find_mut_or_insert_with(0x1004, FindSettings::new(true), || Ok(Some(4)))
            .unwrap();

        assert_eq!(Some((&0x1004, &Some(4))), map.find_left_of(&0x1004, true),);
        assert_eq!(Some((&0x1000, &Some(4))), map.find_left_of(&0x1004, false),);

        assert_eq!(Some((&0x1004, &Some(4))), map.find_right_of(&0x1004, true),);
        assert_eq!(Some((&0x100C, &None)), map.find_right_of(&0x1004, false),);

        assert_eq!(
            map.find_left_of(&0x1004, true),
            map.find_right_of(&0x1004, true),
        );
    }
}
