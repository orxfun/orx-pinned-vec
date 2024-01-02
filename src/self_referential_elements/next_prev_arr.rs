/// Trait for elements that holds a constant-sized array of references to a collection of next elements.
///
/// This can be considered as a constant-sized generalization of the [`crate::self_referential_elements::SelfRefNext`] trait.
/// *Constant number of linked element references is often useful to avoid additional indirection.*
///
/// Notice that the trait itself has a life time `'a` which is equal to the lifetime of the potential linked elements.
/// This is on purpose as the underlying goal of having these traits it to build self-referential-collections
/// where each element live together and share the same lifetime.
pub trait SelfRefNextArr<'a, const N: usize> {
    /// Reference to the slice of next elements of this element.
    fn next_vec(&self) -> &[Option<&'a Self>; N];

    /// Sets next elements of this element to the given `next` collection.
    fn set_next_vec(&mut self, next: [Option<&'a Self>; N]);

    /// Returns a mutable reference to the collection of next elements.
    fn next_mut(&mut self) -> &mut [Option<&'a Self>; N];
}

/// Trait for elements that holds a constant-sized array of references to a collection of prev elements.
///
/// This can be considered as a constant-sized generalization of the [`crate::self_referential_elements::SelfRefPrev`] trait.
/// *Constant number of linked element references is often useful to avoid additional indirection.*
///
/// Notice that the trait itself has a life time `'a` which is equal to the lifetime of the potential linked elements.
/// This is on purpose as the underlying goal of having these traits it to build self-referential-collections
/// where each element live together and share the same lifetime.
pub trait SelfRefPrevArr<'a, const N: usize> {
    /// Reference to the slice of prev elements of this element.
    fn prev_vec(&self) -> &[Option<&'a Self>; N];

    /// Sets prev elements of this element to the given `prev` collection.
    fn set_prev_vec(&mut self, prev: [Option<&'a Self>; N]);

    /// Returns a mutable reference to the collection of prev elements.
    fn prev_mut(&mut self) -> &mut [Option<&'a Self>; N];
}
