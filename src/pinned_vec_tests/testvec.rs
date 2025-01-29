use super::helpers::range::{range_end, range_start};
use crate::*;
use alloc::vec::Vec;
use core::{
    cmp::Ordering,
    iter::Rev,
    ops::{Index, IndexMut, RangeBounds},
};
use orx_iterable::Collection;
use orx_pseudo_default::PseudoDefault;

pub struct TestVec<T>(Vec<T>);

impl<T> PseudoDefault for TestVec<T> {
    fn pseudo_default() -> Self {
        Self(Default::default())
    }
}

impl<T> TestVec<T> {
    #[cfg(test)]
    pub fn new(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    fn assert_has_room(&self, required_additional_space: usize) {
        assert!(PinnedVec::len(self) + required_additional_space <= self.0.capacity())
    }
}

impl<T> Index<usize> for TestVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for TestVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T> IntoIterator for TestVec<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a TestVec<T> {
    type Item = &'a T;

    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut TestVec<T> {
    type Item = &'a mut T;

    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<T> PinnedVec<T> for TestVec<T> {
    type IterRev<'a>
        = Rev<core::slice::Iter<'a, T>>
    where
        T: 'a,
        Self: 'a;
    type IterMutRev<'a>
        = Rev<core::slice::IterMut<'a, T>>
    where
        T: 'a,
        Self: 'a;
    type SliceIter<'a>
        = Option<&'a [T]>
    where
        T: 'a,
        Self: 'a;
    type SliceMutIter<'a>
        = Option<&'a mut [T]>
    where
        T: 'a,
        Self: 'a;

    fn index_of(&self, data: &T) -> Option<usize> {
        crate::utils::slice::index_of(&self.0, data)
    }

    fn index_of_ptr(&self, element_ptr: *const T) -> Option<usize> {
        crate::utils::slice::index_of_ptr(&self.0, element_ptr)
    }

    fn push_get_ptr(&mut self, value: T) -> *const T {
        let idx = self.0.len();
        self.0.push(value);
        unsafe { self.0.as_ptr().add(idx) }
    }

    unsafe fn iter_ptr<'v, 'i>(&'v self) -> impl Iterator<Item = *const T> + 'i
    where
        T: 'i,
    {
        let ptr = self.0.as_ptr();
        (0..self.0.len()).map(move |i| unsafe { ptr.add(i) })
    }

    unsafe fn iter_ptr_rev<'v, 'i>(&'v self) -> impl Iterator<Item = *const T> + 'i
    where
        T: 'i,
    {
        let ptr = self.0.as_ptr();
        (0..self.0.len()).rev().map(move |i| unsafe { ptr.add(i) })
    }

    fn contains_reference(&self, element: &T) -> bool {
        utils::slice::contains_reference(self.0.as_slice(), element)
    }

    fn contains_ptr(&self, element_ptr: *const T) -> bool {
        utils::slice::contains_ptr(self.0.as_slice(), element_ptr)
    }

    fn clear(&mut self) {
        self.0.clear();
    }

    fn capacity(&self) -> usize {
        self.0.capacity()
    }

    fn capacity_state(&self) -> CapacityState {
        CapacityState::FixedCapacity(PinnedVec::capacity(self))
    }

    fn extend_from_slice(&mut self, other: &[T])
    where
        T: Clone,
    {
        self.assert_has_room(other.len());
        self.0.extend_from_slice(other)
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.0.get_mut(index)
    }

    unsafe fn get_unchecked(&self, index: usize) -> &T {
        self.0.get_unchecked(index)
    }

    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        self.0.get_unchecked_mut(index)
    }

    fn first(&self) -> Option<&T> {
        self.0.first()
    }

    fn last(&self) -> Option<&T> {
        self.0.last()
    }

    unsafe fn first_unchecked(&self) -> &T {
        &(self.0)[0]
    }

    unsafe fn last_unchecked(&self) -> &T {
        &(self.0)[PinnedVec::len(self) - 1]
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn push(&mut self, value: T) {
        self.assert_has_room(1);
        self.0.push(value)
    }

    fn insert(&mut self, index: usize, element: T) {
        self.assert_has_room(1);
        self.0.insert(index, element)
    }

    fn remove(&mut self, index: usize) -> T {
        self.0.remove(index)
    }

    fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    fn swap(&mut self, a: usize, b: usize) {
        self.0.swap(a, b)
    }

    fn truncate(&mut self, len: usize) {
        self.0.truncate(len)
    }

    fn iter_rev(&self) -> Self::IterRev<'_> {
        self.0.iter().rev()
    }

    fn iter_mut_rev(&mut self) -> Self::IterMutRev<'_> {
        self.0.iter_mut().rev()
    }

    fn slices<R: RangeBounds<usize>>(&self, range: R) -> Self::SliceIter<'_> {
        let a = range_start(&range);
        let b = range_end(&range, PinnedVec::len(self));

        match b.saturating_sub(a) {
            0 => Some(&[]),
            _ => match (a.cmp(&PinnedVec::len(self)), b.cmp(&PinnedVec::len(self))) {
                (Ordering::Equal | Ordering::Greater, _) => None,
                (_, Ordering::Greater) => None,
                _ => Some(&self.0[a..b]),
            },
        }
    }

    fn slices_mut<R: RangeBounds<usize>>(&mut self, range: R) -> Self::SliceMutIter<'_> {
        let a = range_start(&range);
        let b = range_end(&range, PinnedVec::len(self));

        match b.saturating_sub(a) {
            0 => Some(&mut []),
            _ => match (a.cmp(&PinnedVec::len(self)), b.cmp(&PinnedVec::len(self))) {
                (Ordering::Equal | Ordering::Greater, _) => None,
                (_, Ordering::Greater) => None,
                _ => Some(&mut self.0[a..b]),
            },
        }
    }

    fn get_ptr(&self, index: usize) -> Option<*const T> {
        (index < self.0.capacity()).then(|| unsafe { self.0.as_ptr().add(index) })
    }

    fn get_ptr_mut(&mut self, index: usize) -> Option<*mut T> {
        (index < self.0.capacity()).then(|| unsafe { self.0.as_mut_ptr().add(index) })
    }

    unsafe fn set_len(&mut self, new_len: usize) {
        self.0.set_len(new_len)
    }

    fn binary_search_by<F>(&self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> Ordering,
    {
        self.0.binary_search_by(f)
    }

    fn sort(&mut self)
    where
        T: Ord,
    {
        self.0.sort()
    }

    fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.0.sort_by(compare)
    }

    fn sort_by_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.0.sort_by_key(f)
    }
}
