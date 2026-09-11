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
///
/// The following code does not compile:
///
/// ```ignore
/// fn add_doubles_of_evens(vec: &mut Vec<u32>) {
///     for i in vec.iter().copied() {
///         if i.is_multiple_of(2) {
///             let doubled = 2 * i;
///             vec.push(doubled); // cannot borrow `*vec` as mutable because it is also borrowed as immutable
///         }
///     }
/// }
///
/// let mut vec = vec![9, 10, 11];
///
/// add_doubles_of_evens(&mut vec);
///
/// assert_eq!(&vec, &[9, 10, 11, 20]);
/// ```
///
/// However, this would safely work with a pinned vector.
/// `SplitVec` is one pinned vector implementation, see `orx-split-vec` crate for details.
///
/// ```ignore
/// use orx_pinned_vec::*;
///
/// fn add_doubles_of_evens(vec: &mut SplitVec<u32>) {
///     let vec = vec.as_imp_vec();
///     for i in vec.iter().copied() {
///         if i.is_multiple_of(2) {
///             let doubled = 2 * i;
///             vec.imp_push(doubled);
///         }
///     }
/// }
///
/// let mut vec = SplitVec::new();
/// vec.extend_from_slice(&[9, 10, 11]);
///
/// add_doubles_of_evens(&mut vec);
///
/// assert_eq!(&vec, &[9, 10, 11, 20]);
/// ```
pub struct ImpVec<T, P, S>
where
    P: PinnedVec<T>,
    S: SoM<P>,
{
    pinned_vec: UnsafeCell<S>,
    phantom: PhantomData<(T, P)>,
}

impl<T, P, S> ImpVec<T, P, S>
where
    P: PinnedVec<T>,
    S: SoM<P>,
{
    // helper

    #[allow(clippy::mut_from_ref)]
    #[inline(always)]
    fn pinned_mut(&self) -> &mut P {
        // SAFETY: `ImpVec` does not implement Send or Sync.
        // Further `imp_push` and `imp_extend_from_slice` methods are safe to call with a shared reference due to pinned vector guarantees.
        // All other calls to this internal method require a mutable reference.
        unsafe { &mut *self.pinned_vec.get() }.get_mut()
    }

    #[inline(always)]
    fn pinned(&self) -> &P {
        // SAFETY: `ImpVec` does not implement Send or Sync.
        // Further `imp_push` and `imp_extend_from_slice` methods are safe to call with a shared reference due to pinned vector guarantees.
        // All other calls to this internal method require a mutable reference.
        unsafe { &*self.pinned_vec.get() }.get_ref()
    }

    // new

    pub(super) fn new(pinned_vec: S) -> Self {
        Self {
            pinned_vec: pinned_vec.into(),
            phantom: PhantomData,
        }
    }

    // api

    /// Returns back the inner pinned vector that this `ImpVec` is created from.
    pub fn into_inner(self) -> S {
        self.pinned_vec.into_inner()
    }

    /// Pushes the `value` to the vector.
    /// This method differs from the `push` method with the required reference.
    /// Unlike `push`, `imp_push` allows to push the element with a shared reference.
    ///
    /// # Example
    ///
    /// ```rust ignore
    /// use pinned_vec::*;
    ///
    /// let mut split_vec = SplitVec::new();
    ///
    /// let mut vec = split_vec.as_imp_vec();
    ///
    /// // regular push with &mut self
    /// vec.push(42);
    ///
    /// // hold on to a reference to the first element
    /// let ref_to_first = &vec[0];
    /// assert_eq!(ref_to_first, &42);
    ///
    /// // imp_push with &self
    /// vec.imp_push(7);
    ///
    /// // due to `PinnedVec` guarantees, this push will never invalidate prior references
    /// assert_eq!(ref_to_first, &42);
    /// ```
    #[inline(always)]
    pub fn imp_push(&self, value: T) {
        self.pinned_mut().push(value);
    }

    /// Pushes the `value` to the vector and returns a reference to it.
    ///
    /// It is the composition of [`vec.imp_push(value)`] call followed by `&vec[vec.len() - 1]`.
    ///
    /// [`vec.imp_push(value)`]: crate::ImpVec::imp_push
    ///
    /// # Examples
    ///
    /// This method provides a shorthand for the following common use case.
    ///
    /// ```rust ignore
    /// use pinned_vec::*;
    ///
    /// let mut split_vec = SplitVec::new();
    ///
    /// let mut vec = split_vec.as_imp_vec();
    ///
    /// vec.imp_push('a');
    /// let a = &vec[vec.len() - 1];
    /// assert_eq!(a, &'a');
    ///
    /// // or with imp_push_get_ref
    ///
    /// let b = vec.imp_push_get_ref('b');
    /// assert_eq!(b, &'b');
    /// ```
    #[inline(always)]
    pub fn imp_push_get_ref(&self, value: T) -> &T {
        let pinned = self.pinned_mut();
        pinned.push(value);
        &pinned[pinned.len() - 1]
    }

    /// Extends the vector with the given `slice`.
    /// This method differs from the `extend_from_slice` method with the required reference.
    /// Unlike `extend_from_slice`, `imp_extend_from_slice` allows to push the element with a shared reference.
    ///
    /// # Example
    ///
    /// ```rust ignore
    /// use pinned_vec::*;
    ///
    /// let mut split_vec = SplitVec::new();
    ///
    /// // regular extend_from_slice with &mut self
    /// vec.extend_from_slice(&[42]);
    ///
    /// // hold on to a reference to the first element
    /// let ref_to_first = &vec[0];
    /// assert_eq!(ref_to_first, &42);
    ///
    /// // imp_extend_from_slice with &self
    /// vec.imp_extend_from_slice(&[0, 1, 2, 3]);
    /// assert_eq!(vec.len(), 5);
    ///
    /// // due to `PinnedVec` guarantees, this extend will never invalidate prior references
    /// assert_eq!(ref_to_first, &42);
    /// ```
    pub fn imp_extend_from_slice(&self, slice: &[T])
    where
        T: Clone,
    {
        self.pinned_mut().extend_from_slice(slice);
    }
}

impl<T, P, S> Deref for ImpVec<T, P, S>
where
    P: PinnedVec<T>,
    S: SoM<P>,
{
    type Target = P;
    fn deref(&self) -> &Self::Target {
        self.pinned()
    }
}

impl<T, P, S> DerefMut for ImpVec<T, P, S>
where
    P: PinnedVec<T>,
    S: SoM<P>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.pinned_mut()
    }
}
