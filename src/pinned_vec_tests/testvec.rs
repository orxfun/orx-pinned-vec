use crate::*;
use std::{cmp::Ordering, iter::Rev, ops::RangeBounds};

use super::helpers::range::{range_end, range_start};

pub struct TestVec<T>(Vec<T>);

impl<T> TestVec<T> {
    #[cfg(test)]
    pub fn new(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    fn assert_has_room(&self, required_additional_space: usize) {
        assert!(self.len() + required_additional_space <= self.0.capacity())
    }
}

impl<T> IntoIterator for TestVec<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> PinnedVec<T> for TestVec<T> {
    type Iter<'a> = std::slice::Iter<'a, T> where T: 'a, Self: 'a;
    type IterMut<'a> = std::slice::IterMut<'a, T> where T: 'a, Self: 'a;
    type IterRev<'a> = Rev<std::slice::Iter<'a, T>> where T: 'a, Self: 'a;
    type IterMutRev<'a> = Rev<std::slice::IterMut<'a, T>> where T: 'a, Self: 'a;
    type SliceIter<'a> = Option<&'a [T]> where T: 'a, Self: 'a;
    type SliceMutIter<'a> = Option<&'a mut [T]> where T: 'a, Self: 'a;

    fn index_of(&self, data: &T) -> Option<usize> {
        crate::utils::slice::index_of(&self.0, data)
    }

    fn contains_reference(&self, element: &T) -> bool {
        self.index_of(element).is_some()
    }

    fn clear(&mut self) {
        self.0.clear();
    }

    fn capacity(&self) -> usize {
        self.0.capacity()
    }

    fn capacity_state(&self) -> CapacityState {
        CapacityState::FixedCapacity(self.capacity())
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
        &(self.0)[self.len() - 1]
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

    fn iter(&self) -> Self::Iter<'_> {
        self.0.iter()
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.0.iter_mut()
    }

    fn iter_rev(&self) -> Self::IterRev<'_> {
        self.0.iter().rev()
    }

    fn iter_mut_rev(&mut self) -> Self::IterMutRev<'_> {
        self.0.iter_mut().rev()
    }

    fn slices<R: RangeBounds<usize>>(&self, range: R) -> Self::SliceIter<'_> {
        let a = range_start(&range);
        let b = range_end(&range, self.len());

        match b.saturating_sub(a) {
            0 => Some(&[]),
            _ => match (a.cmp(&self.len()), b.cmp(&self.len())) {
                (Ordering::Equal | Ordering::Greater, _) => None,
                (_, Ordering::Greater) => None,
                _ => Some(&self.0[a..b]),
            },
        }
    }

    fn slices_mut<R: RangeBounds<usize>>(&mut self, range: R) -> Self::SliceMutIter<'_> {
        let a = range_start(&range);
        let b = range_end(&range, self.len());

        match b.saturating_sub(a) {
            0 => Some(&mut []),
            _ => match (a.cmp(&self.len()), b.cmp(&self.len())) {
                (Ordering::Equal | Ordering::Greater, _) => None,
                (_, Ordering::Greater) => None,
                _ => Some(&mut self.0[a..b]),
            },
        }
    }

    unsafe fn get_ptr_mut(&mut self, index: usize) -> Option<*mut T> {
        if index < self.0.capacity() {
            Some(self.0.as_mut_ptr().add(index))
        } else {
            None
        }
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

    fn try_grow(&mut self) -> Result<usize, PinnedVecGrowthError> {
        Err(PinnedVecGrowthError::FailedToGrowWhileKeepingElementsPinned)
    }

    unsafe fn grow_to(
        &mut self,
        new_capacity: usize,
        _: bool,
    ) -> Result<usize, PinnedVecGrowthError> {
        match self.capacity() {
            current_capacity if current_capacity >= new_capacity => Ok(current_capacity),
            _ => Err(PinnedVecGrowthError::FailedToGrowWhileKeepingElementsPinned),
        }
    }

    fn grow_and_initialize<F>(
        &mut self,
        new_min_len: usize,
        f: F,
    ) -> Result<usize, PinnedVecGrowthError>
    where
        F: Fn() -> T,
    {
        let prior_len = self.len();
        unsafe { self.grow_to(new_min_len, false) }.map(|capacity| {
            debug_assert!(capacity >= new_min_len);
            for _ in prior_len..capacity {
                self.push(f());
            }
            debug_assert_eq!(self.len(), capacity);
            capacity
        })
    }

    unsafe fn concurrently_grow_to(
        &mut self,
        _: usize,
        _: bool,
    ) -> Result<usize, PinnedVecGrowthError> {
        Err(PinnedVecGrowthError::FailedToGrowWhileKeepingElementsPinned)
    }

    fn try_reserve_maximum_concurrent_capacity(
        &mut self,
        _new_maximum_capacity: usize,
    ) -> Result<usize, String> {
        Err("cannot reserve".to_string())
    }
}
