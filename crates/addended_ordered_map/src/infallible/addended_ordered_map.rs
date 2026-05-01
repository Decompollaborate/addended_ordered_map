/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use alloc::collections::btree_map;
use core::{convert::Infallible, fmt, ops::RangeBounds};

use super::{AddendableKey, SizedValue};
use crate::fallible::AddendedOrderedMapFallible;
use crate::FindSettings;

pub type Range<'a, K, V> = btree_map::Range<'a, K, V>;
pub type RangeMut<'a, K, V> = btree_map::RangeMut<'a, K, V>;

pub struct AddendedOrderedMap<K, V, SIZE>
where
    K: Ord + AddendableKey<SIZE>,
    V: SizedValue<SIZE>,
{
    inner: AddendedOrderedMapFallible<K, V, SIZE, Infallible>,
}

impl<K, V, SIZE> AddendedOrderedMap<K, V, SIZE>
where
    K: Ord + AddendableKey<SIZE>,
    V: SizedValue<SIZE>,
{
    pub fn new() -> Self {
        Self {
            inner: AddendedOrderedMapFallible::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<K, V, SIZE> AddendedOrderedMap<K, V, SIZE>
where
    K: Ord + AddendableKey<SIZE>,
    V: SizedValue<SIZE>,
{
    #[must_use = "This is a lookup function, there are no side-effects on the mapping."]
    pub fn find(&self, key: &K, settings: FindSettings) -> Option<(&K, &V)> {
        self.inner
            .find(key, settings)
            .expect("Infallible operation")
    }

    #[must_use = "This is a lookup function, there are no side-effects on the mapping."]
    pub fn find_key(&self, key: &K, settings: FindSettings) -> Option<&K> {
        self.inner
            .find_key(key, settings)
            .expect("Infallible operation")
    }

    #[must_use = "This is a lookup function, there are no side-effects on the mapping."]
    pub fn find_value(&self, key: &K, settings: FindSettings) -> Option<&V> {
        self.inner
            .find_value(key, settings)
            .expect("Infallible operation")
    }

    #[must_use = "This is a lookup function, there are no side-effects on the mapping."]
    pub fn find_mut(&mut self, key: &K, settings: FindSettings) -> Option<(&K, &mut V)> {
        self.inner
            .find_mut(key, settings)
            .expect("Infallible operation")
    }

    #[must_use = "This is a lookup function, there are no side-effects on the mapping."]
    pub fn find_left_of(&self, key: &K, inclusive: bool) -> Option<(&K, &V)> {
        self.inner.find_left_of(key, inclusive)
    }

    #[must_use = "This is a lookup function, there are no side-effects on the mapping."]
    pub fn find_right_of(&self, key: &K, inclusive: bool) -> Option<(&K, &V)> {
        self.inner.find_right_of(key, inclusive)
    }

    #[must_use = "This is a lookup function, there are no side-effects on the mapping."]
    pub fn find_left_of_mut(&mut self, key: &K, inclusive: bool) -> Option<(&K, &mut V)> {
        self.inner.find_left_of_mut(key, inclusive)
    }

    #[must_use = "This is a lookup function, there are no side-effects on the mapping."]
    pub fn find_right_of_mut(&mut self, key: &K, inclusive: bool) -> Option<(&K, &mut V)> {
        self.inner.find_right_of_mut(key, inclusive)
    }
}

impl<K, V, SIZE> AddendedOrderedMap<K, V, SIZE>
where
    K: Ord + AddendableKey<SIZE>,
    V: SizedValue<SIZE>,
{
    pub fn find_mut_or_insert_with<F>(
        &mut self,
        key: K,
        settings: FindSettings,
        default: F,
    ) -> (&mut V, bool)
    where
        K: Copy,
        F: FnOnce() -> V,
    {
        self.inner
            .find_mut_or_insert_with(key, settings, || Ok(default()))
            .expect("Infallible operation")
    }

    pub fn find_mut_or_insert_with_key_value<F>(
        &mut self,
        key: &K,
        settings: FindSettings,
        default: F,
    ) -> (&mut V, bool)
    where
        F: FnOnce() -> (K, V),
    {
        self.inner
            .find_mut_or_insert_with_key_value(key, settings, || Ok(default()))
            .expect("Infallible operation")
    }
}

impl<K, V, SIZE> AddendedOrderedMap<K, V, SIZE>
where
    K: Ord + AddendableKey<SIZE>,
    V: SizedValue<SIZE>,
{
    pub fn contains_key_exact(&self, key: &K) -> bool {
        self.inner.contains_key_exact(key)
    }

    pub fn pop_exact(&mut self, key: &K) -> Option<(K, V)> {
        self.inner.pop_exact(key)
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

impl<K, V, SIZE> AddendedOrderedMap<K, V, SIZE>
where
    K: Ord + AddendableKey<SIZE>,
    V: SizedValue<SIZE>,
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

impl<K, V, SIZE> fmt::Debug for AddendedOrderedMap<K, V, SIZE>
where
    K: fmt::Debug + Ord + AddendableKey<SIZE>,
    V: fmt::Debug + SizedValue<SIZE>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Manually implement Debug to hide the `inner` indirection
        write!(f, "AddendedOrderedMap {:?}", self.inner)
    }
}

impl<K, V, SIZE> Clone for AddendedOrderedMap<K, V, SIZE>
where
    K: Clone + Ord + AddendableKey<SIZE>,
    V: Clone + SizedValue<SIZE>,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<K, V, SIZE> core::hash::Hash for AddendedOrderedMap<K, V, SIZE>
where
    K: core::hash::Hash + Ord + AddendableKey<SIZE>,
    V: core::hash::Hash + SizedValue<SIZE>,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<K, V, SIZE> PartialEq for AddendedOrderedMap<K, V, SIZE>
where
    K: PartialEq + Ord + AddendableKey<SIZE>,
    V: PartialEq + SizedValue<SIZE>,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<K, V, SIZE> Eq for AddendedOrderedMap<K, V, SIZE>
where
    K: Eq + Ord + AddendableKey<SIZE>,
    V: Eq + SizedValue<SIZE>,
{
}

impl<K, V, SIZE> PartialOrd for AddendedOrderedMap<K, V, SIZE>
where
    K: PartialOrd + Ord + AddendableKey<SIZE>,
    V: PartialOrd + SizedValue<SIZE>,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<K, V, SIZE> Ord for AddendedOrderedMap<K, V, SIZE>
where
    K: Ord + AddendableKey<SIZE>,
    V: Ord + SizedValue<SIZE>,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<K, V, SIZE> Default for AddendedOrderedMap<K, V, SIZE>
where
    K: Ord + AddendableKey<SIZE>,
    V: SizedValue<SIZE>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, K, V, SIZE> IntoIterator for &'a AddendedOrderedMap<K, V, SIZE>
where
    K: Ord + AddendableKey<SIZE>,
    V: SizedValue<SIZE>,
{
    type Item = (&'a K, &'a V);
    type IntoIter = btree_map::Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/*
impl<'a, K, V, SIZE> IntoIterator for &'a mut AddendedOrderedMap<K, V, SIZE>
{
    type Item = (&'a K, &'a mut V);
    type IntoIter = btree_map::IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
*/

impl<K, V, SIZE> IntoIterator for AddendedOrderedMap<K, V, SIZE>
where
    K: Ord + AddendableKey<SIZE>,
    V: SizedValue<SIZE>,
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

    impl SizedValue<u32> for Option<u32> {
        fn size(&self) -> u32 {
            self.unwrap_or(1)
        }
    }

    #[test]
    fn check_bounds() {
        let mut map: AddendedOrderedMap<u32, Option<u32>, u32> = AddendedOrderedMap::new();

        map.find_mut_or_insert_with(0x100C, FindSettings::new(true), || None);
        map.find_mut_or_insert_with(0x1000, FindSettings::new(true), || Some(4));
        map.find_mut_or_insert_with(0x1004, FindSettings::new(true), || Some(4));

        assert_eq!(
            Some((&0x1000, &Some(4))),
            map.find(&0x1000, FindSettings::new(true)),
        );

        assert_eq!(
            Some((&0x1000, &Some(4))),
            map.find(&0x1002, FindSettings::new(true)),
        );

        assert_eq!(None, map.find(&0x0F00, FindSettings::new(true)),);

        assert_eq!(None, map.find(&0x2000, FindSettings::new(true)),);

        assert_eq!(None, map.find(&0x1002, FindSettings::new(false)),);

        assert_eq!(None, map.find(&0x1008, FindSettings::new(true)),);
    }

    #[test]
    fn check_left_right() {
        let mut map: AddendedOrderedMap<u32, Option<u32>, u32> = AddendedOrderedMap::new();

        map.find_mut_or_insert_with(0x100C, FindSettings::new(true), || None);
        map.find_mut_or_insert_with(0x1000, FindSettings::new(true), || Some(4));
        map.find_mut_or_insert_with(0x1004, FindSettings::new(true), || Some(4));

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
