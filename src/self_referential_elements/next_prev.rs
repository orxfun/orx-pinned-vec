/// Trait for elements that optionally holds a reference to a `next` element.
///
/// A common example is the linked-list where each node holds a reference to the next node.
///
/// Notice that the trait itself has a life time `'a` which is equal to the lifetime of the potential next elements.
/// This is on purpose as the underlying goal of having these traits it to build self-referential-collections
/// where each element live together and share the same lifetime.
///
/// # Example
///
/// ```rust
/// use orx_pinned_vec::self_referential_elements::*;
///
/// #[derive(PartialEq, Debug)]
/// struct LinkedListNode<'a, Data> {
///     data: Data,
///     next: Option<&'a Self>,
///     prev: Option<&'a Self>,
/// }
///
/// impl<'a, Data> SelfRefNext<'a> for LinkedListNode<'a, Data> {
///     fn next(&self) -> Option<&'a Self> {
///         self.next
///     }
///     fn set_next(&mut self, next: Option<&'a Self>) {
///         self.next = next;
///     }
/// }
///
/// let mut a = LinkedListNode {
///     data: 'a',
///     next: None,
///     prev: None,
/// };
/// assert_eq!(a.next(), None);
///
/// let b = LinkedListNode {
///     data: 'b',
///     next: None,
///     prev: None,
/// };
/// a.set_next(Some(&b));
/// assert_eq!(a.next(), Some(&b));
///
/// a.set_next(None);
/// assert_eq!(a.next(), None);
/// ```
pub trait SelfRefNext<'a> {
    /// Reference to the next element of this element; None if this element does not have a next.
    fn next(&self) -> Option<&'a Self>;

    /// Sets next of this element to the given `next`, which is a reference to the element to be linked.
    fn set_next(&mut self, next: Option<&'a Self>);
}

/// Trait for elements that optionally holds a reference to a `prev` element.
///
/// A common example is the doubly-linked-list where each node holds a reference to the prev node.
///
/// Notice that the trait itself has a life time `'a` which is equal to the lifetime of the potential previous elements.
/// This is on purpose as the underlying goal of having these traits it to build self-referential-collections
/// where each element live together and share the same lifetime.
///
/// # Example
///
/// ```rust
/// use orx_pinned_vec::self_referential_elements::*;
///
/// #[derive(PartialEq, Debug)]
/// struct LinkedListNode<'a, Data> {
///     data: Data,
///     next: Option<&'a Self>,
///     prev: Option<&'a Self>,
/// }
///
/// impl<'a, Data> SelfRefPrev<'a> for LinkedListNode<'a, Data> {
///     fn prev(&self) -> Option<&'a Self> {
///         self.prev
///     }
///     fn set_prev(&mut self, prev: Option<&'a Self>) {
///         self.prev = prev;
///     }
/// }
///
/// let mut a = LinkedListNode {
///     data: 'a',
///     next: None,
///     prev: None,
/// };
/// assert_eq!(a.prev(), None);
///
/// let b = LinkedListNode {
///     data: 'b',
///     next: None,
///     prev: None,
/// };
/// a.set_prev(Some(&b));
/// assert_eq!(a.prev(), Some(&b));
///
/// a.set_prev(None);
/// assert_eq!(a.prev(), None);
/// ```
pub trait SelfRefPrev<'a> {
    /// Reference to the next element of this element; None if this element does not have a next.
    fn prev(&self) -> Option<&'a Self>;

    /// Sets next of this element to the given `next`, which is a reference to the element to be linked.
    fn set_prev(&mut self, next: Option<&'a Self>);
}
