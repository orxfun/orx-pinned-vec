use crate::PinnedVec;

/// Tests the pinned vector guarantees of the specific `PinnedVec` implementation `P`. Assertions fail if any of the guarantees are not satisfied.
///
/// To be specific on the required guarantees, let's assume that a pinned vector `pinned` currently has `n` elements:
///
/// * `pinned.push(new_element)`: does not change the memory locations of the first `n` elements;
/// * `pinned.extend_from_slice(slice)`: does not change the memory locations of the first `n` elements;
/// * `pinned.insert(a, new_element)`: does not change the memory locations of the first `a` elements, where `a <= n`; elements to the right of the inserted element might be changed (commonly shifted to right).
/// * `pinned.pop()`: does not change the memory locations of the first `n-1` elements (the n-th element will be removed);
/// * `pinned.remove(a)`: does not change the memory locations of the first `a` elements, where `a < n`; elements to the right of the removed element might be changed (commonly shifted to left).
/// * `pinned.truncate(a)`: does not change the memory locations of the first `a` elements, where `a < n`.
pub fn test_pinned_vec<P: PinnedVec<usize>>(pinned_vec: P, test_vec_len: usize) {
    let pinned_vec = super::push::push(pinned_vec, test_vec_len);
    let pinned_vec = super::extend::extend(pinned_vec, test_vec_len);
    let pinned_vec = super::insert::insert(pinned_vec, test_vec_len);
    let pinned_vec = super::pop::pop(pinned_vec, test_vec_len);
    let pinned_vec = super::remove::remove(pinned_vec, test_vec_len);
    let pinned_vec = super::truncate::truncate(pinned_vec, test_vec_len);
    let pinned_vec = super::slices::slices(pinned_vec, test_vec_len);
    let pinned_vec = super::binary_search::binary_search(pinned_vec, test_vec_len);
    let _ = super::unsafe_writer::unsafe_writer(pinned_vec, test_vec_len);
}

#[cfg(test)]
mod tests {

    use orx_pseudo_default::PseudoDefault;

    use super::*;
    use crate::{
        pinned_vec_tests::helpers::range::{range_end, range_start},
        CapacityState,
    };
    use std::{cmp::Ordering, iter::Rev, ops::RangeBounds};

    #[derive(Debug)]
    struct JustVec<T>(Vec<T>);

    impl<T> PseudoDefault for JustVec<T> {
        fn pseudo_default() -> Self {
            Self(Default::default())
        }
    }

    impl<T> JustVec<T> {
        fn assert_has_room(&self, required_additional_space: usize) {
            assert!(PinnedVec::len(self) + required_additional_space <= self.0.capacity())
        }
    }

    impl<T> IntoIterator for JustVec<T> {
        type Item = T;
        type IntoIter = <Vec<T> as IntoIterator>::IntoIter;
        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }

    impl<T> PinnedVec<T> for JustVec<T> {
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
    }

    #[test]
    fn empty_vec_passes() {
        let vec = JustVec(vec![]);
        test_pinned_vec(vec, 0);
    }

    #[test]
    fn within_capacity_vec_passes() {
        let capacity = 129;
        let vec = JustVec(Vec::with_capacity(capacity));
        test_pinned_vec(vec, capacity);
    }

    #[test]
    #[should_panic]
    fn capacity_exceeding_vec_fails() {
        // not necessarily fails in every expansion, but will eventually fail.
        let lengths = [8, 32, 1025];
        for len in lengths {
            let vec = JustVec(Vec::with_capacity(0));
            test_pinned_vec(vec, len);
        }
    }
}
