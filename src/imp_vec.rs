use crate::PinnedVec;
use core::ops::{Deref, DerefMut};
use core::{cell::UnsafeCell, marker::PhantomData};
use orx_self_or::SoM;

/// `ImpVec`, stands for immutable push vector 👿, is a data structure which allows appending elements with a shared reference.
///
/// Specifically, it extends vector capabilities with the following three methods:
///
/// * `fn imp_push(&self, value: T)`
/// * `fn imp_extend_from_slice(&self, slice: &[T])`
/// * `fn imp_push_get_ref(&self, value: T) -> &T`
///
/// Note that both of these methods can be called with `&self` rather than `&mut self`.
/// This is safe since growth does not cause memory locations of existing elements of pinned vectors.
///
/// # Examples
///
/// A common use case is when we want to iterate over existing elements,
/// and add new elements to the same vector.
pub struct ImpVec<T, P>
where
    P: PinnedVec<T>,
{
    pinned_vec: UnsafeCell<P>,
    phantom: PhantomData<T>,
}

impl<T, P> ImpVec<T, P>
where
    P: PinnedVec<T>,
{
    // helper

    #[allow(clippy::mut_from_ref)]
    #[inline(always)]
    fn pinned_mut(&self) -> &mut P {
        // SAFETY: `ImpVec` does not implement Send or Sync.
        // Further `imp_push` and `imp_extend_from_slice` methods are safe to call with a shared reference due to pinned vector guarantees.
        // All other calls to this internal method require a mutable reference.
        unsafe { &mut *self.pinned_vec.get() }
    }

    #[inline(always)]
    fn pinned(&self) -> &P {
        // SAFETY: `ImpVec` does not implement Send or Sync.
        // Further `imp_push` and `imp_extend_from_slice` methods are safe to call with a shared reference due to pinned vector guarantees.
        // All other calls to this internal method require a mutable reference.
        unsafe { &*self.pinned_vec.get() }
    }

    // new

    pub(super) fn new(pinned_vec: P) -> Self {
        Self {
            pinned_vec: pinned_vec.into(),
            phantom: PhantomData,
        }
    }

    // api

    pub fn into_inner(self) -> P {
        self.pinned_vec.into_inner()
    }

    #[inline(always)]
    pub fn imp_push(&self, value: T) {
        self.pinned_mut().push(value);
    }

    #[inline(always)]
    pub fn imp_push_get_ref(&self, value: T) -> &T {
        let pinned = self.pinned_mut();
        pinned.push(value);
        &pinned[pinned.len() - 1]
    }

    pub fn imp_extend_from_slice(&self, slice: &[T])
    where
        T: Clone,
    {
        self.pinned_mut().extend_from_slice(slice);
    }
}

impl<T, P: PinnedVec<T>> Deref for ImpVec<T, P> {
    type Target = P;
    fn deref(&self) -> &Self::Target {
        self.pinned()
    }
}

impl<T, P: PinnedVec<T>> DerefMut for ImpVec<T, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.pinned_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::{PinnedVec, pinned_vec_tests::testvec::FixedCapVec};
    use orx_iterable::Collection;

    #[test]
    fn xyz() {
        let mut vec = vec![1, 2, 3];

        fn add_doubles_of_evens(vec: &mut Vec<u32>) {
            //
        }
    }

    #[test]
    fn abc() {
        let mut vec = FixedCapVec::new(10);

        vec.push(0);
        vec.push(1);
        vec.push(2);

        let imp = vec.into_imp_vec();

        for x in imp.iter().copied() {
            imp.imp_push(x);
        }

        imp.imp_push(3);
        imp.imp_push(4);
        imp.imp_push(5);

        let vec = imp.into_inner();

        assert_eq!(vec, FixedCapVec::<i32>::new(1));
    }

    #[test]
    fn def() {
        struct MyStr {
            vec: FixedCapVec<usize>,
        }
    }
}
