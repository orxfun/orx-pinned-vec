use orx_iterable::Collection;
use orx_pinned_vec::*;
use orx_pseudo_default::PseudoDefault;
use std::{
    cmp::Ordering,
    iter::Rev,
    ops::{Bound, Index, IndexMut, RangeBounds},
};

pub struct StdVec<T>(Vec<T>);

impl<T> PseudoDefault for StdVec<T> {
    fn pseudo_default() -> Self {
        Self(Default::default())
    }
}

impl<T> StdVec<T> {
    #[cfg(test)]
    pub fn new(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}

impl<T> Index<usize> for StdVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for StdVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T> IntoIterator for StdVec<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a StdVec<T> {
    type Item = &'a T;

    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut StdVec<T> {
    type Item = &'a mut T;

    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<T> PinnedVec<T> for StdVec<T> {
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
        self.0.extend_from_slice(other)
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.0.get_mut(index)
    }

    unsafe fn get_unchecked(&self, index: usize) -> &T {
        unsafe { self.0.get_unchecked(index) }
    }

    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        unsafe { self.0.get_unchecked_mut(index) }
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
        self.0.push(value)
    }

    fn insert(&mut self, index: usize, element: T) {
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

    fn iter_over<'a>(
        &'a self,
        range: impl RangeBounds<usize>,
    ) -> impl ExactSizeIterator<Item = &'a T>
    where
        T: 'a,
    {
        use core::cmp::{max, min};

        let len = PinnedVec::len(self);
        let a = min(len, range_start(&range));
        let b = max(a, min(len, range_end(&range, len)));

        self.0[a..b].iter()
    }

    fn iter_mut_over<'a>(
        &'a mut self,
        range: impl RangeBounds<usize>,
    ) -> impl ExactSizeIterator<Item = &'a mut T>
    where
        T: 'a,
    {
        use core::cmp::{max, min};

        let len = PinnedVec::len(self);
        let a = min(len, range_start(&range));
        let b = max(a, min(len, range_end(&range, len)));

        self.0[a..b].iter_mut()
    }

    fn get_ptr(&self, index: usize) -> Option<*const T> {
        (index < self.0.capacity()).then(|| unsafe { self.0.as_ptr().add(index) })
    }

    fn get_ptr_mut(&mut self, index: usize) -> Option<*mut T> {
        (index < self.0.capacity()).then(|| unsafe { self.0.as_mut_ptr().add(index) })
    }

    unsafe fn set_len(&mut self, new_len: usize) {
        unsafe { self.0.set_len(new_len) }
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

    fn capacity_bound(&self) -> usize {
        usize::MAX
    }
}

fn range_start<R: RangeBounds<usize>>(range: &R) -> usize {
    match range.start_bound() {
        Bound::Excluded(x) => x + 1,
        Bound::Included(x) => *x,
        Bound::Unbounded => 0,
    }
}
fn range_end<R: RangeBounds<usize>>(range: &R, vec_len: usize) -> usize {
    match range.end_bound() {
        Bound::Excluded(x) => *x,
        Bound::Included(x) => x + 1,
        Bound::Unbounded => vec_len,
    }
}

// PINNED ELEMENT TESTS

#[cfg(not(miri))]
const N: usize = 64 * 1024;
#[cfg(miri)]
const N: usize = 56;

#[test]
fn std_vec_extend_with_capacity() {
    pinned_vec_tests::extend(StdVec(Vec::with_capacity(N)), N);
}

#[test]
#[should_panic]
fn std_vec_extend() {
    pinned_vec_tests::extend(StdVec(Vec::new()), N);
}

#[test]
fn std_vec_insert_with_capacity() {
    pinned_vec_tests::insert(StdVec(Vec::with_capacity(N)), N);
}

#[test]
#[should_panic]
fn std_vec_insert() {
    pinned_vec_tests::insert(StdVec(Vec::new()), N);
}

#[test]
fn std_vec_pop_with_capacity() {
    pinned_vec_tests::pop(StdVec(Vec::with_capacity(N)), N);
}

#[test]
#[should_panic]
fn std_vec_pop() {
    pinned_vec_tests::pop(StdVec(Vec::new()), N);
}

#[test]
fn std_vec_push_with_capacity() {
    pinned_vec_tests::push(StdVec(Vec::with_capacity(N)), N);
}

#[test]
#[should_panic]
fn std_vec_push() {
    pinned_vec_tests::push(StdVec(Vec::new()), N);
}

#[test]
fn std_vec_remove_with_capacity() {
    pinned_vec_tests::remove(StdVec(Vec::with_capacity(N)), N);
}

#[test]
#[should_panic]
fn std_vec_remove() {
    pinned_vec_tests::remove(StdVec(Vec::new()), N);
}

#[test]
fn std_vec_truncate_with_capacity() {
    pinned_vec_tests::truncate(StdVec(Vec::with_capacity(N)), N);
}

#[test]
#[should_panic]
fn std_vec_truncate() {
    pinned_vec_tests::truncate(StdVec(Vec::new()), N);
}

#[test]
fn std_vec_all_with_capacity() {
    pinned_vec_tests::test_pinned_vec(StdVec(Vec::with_capacity(N)), N);
}

#[test]
#[should_panic]
fn std_vec_all() {
    pinned_vec_tests::test_pinned_vec(StdVec(Vec::new()), N);
}
