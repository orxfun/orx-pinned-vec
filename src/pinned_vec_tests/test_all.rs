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
    let _ = super::truncate::truncate(pinned_vec, test_vec_len);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct JustVec<T>(Vec<T>);
    impl<T> PinnedVec<T> for JustVec<T> {
        type Iter<'a> = std::slice::Iter<'a, T> where T: 'a, Self: 'a;
        type IterMut<'a> = std::slice::IterMut<'a, T> where T: 'a, Self: 'a;
        type IterRev<'a> =std::iter:: Rev<std::slice::Iter<'a, T>> where T: 'a, Self: 'a;
        type IterMutRev<'a> =std::iter:: Rev<std::slice::IterMut<'a, T>> where T: 'a, Self: 'a;

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

        unsafe fn set_len(&mut self, new_len: usize) {
            self.0.set_len(new_len)
        }
    }

    #[test]
    fn empty_vec_passes() {
        let vec = JustVec(vec![]);
        test_pinned_vec(vec, 0);
    }

    #[test]
    fn within_capacity_vec_passes() {
        let capacity = 1024 * 4;
        let vec = JustVec(Vec::with_capacity(capacity));
        test_pinned_vec(vec, capacity);
    }

    #[test]
    #[should_panic]
    fn capacity_exceeding_vec_fails() {
        // not necessarily fails in every expansion, but will eventually fail.
        let lengths = [8, 32, 1024, 32768];
        for len in lengths {
            let vec = JustVec(Vec::with_capacity(0));
            test_pinned_vec(vec, len);
        }
    }
}
