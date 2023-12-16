#[allow(unused_variables)]

/// A type to be used as an element of a `PinnedVec` which holds a
/// reference to a another element of the same vector.
///
/// These data types are particularly useful for data types defining
/// relations about its children such as trees or graphs.
///
/// # Example
///
/// Consider the following type which is common to tree structures:
///
/// ```rust
/// struct TreeNode<'a, T> {
///     value: T,
///     parent: Option<&'a TreeNode<'a, T>>,
/// }
/// ```
///
/// Further assume that we want to keep all nodes of the tree in the same vector.
/// Compared to alternatives, this is helpful at least for the following reasons:
///
/// * keeping all nodes together helps in achieving better cache locality,
/// * the references defining the tree structure are thin rather than wide pointers (box-rc-free),
/// * requires less heap allocations: only the vector is allocated together with all its elements,
/// as opposed to allocating each node separately in an arbitrary memory location.
///
/// # Safety
/// On the other hand, such data structures require more care about safety and correctness.
/// Since each vector element can hold a reference to another vector element,
/// `mut` methods need to be investigated carefully.
///
/// Consider for instance a vector of two nodes, say `a` and `b`,
/// each has the other as the `parent`; i.e., defining a cyclic relationship `a <--> b`.
/// Assume that we `insert` another node at the beginning of this vector, say `x`,
/// resulting in the vector `[ x, a, b ]`.
/// Now `a`'s parent appears to be itself (position 1);
/// and `b`'s parent appears to be `x` (position 0).
/// This is an undefined behavior (UB).
///
/// Alternatively, if we call `remove(1)` on this vector, we end up with the vector `[ a ]`.
/// Now `a` is pointing to a memory location that does not belong to this vector any more.
/// We again end up with UB.
///
/// We can conclude that all mut methods causing either of the following two lead to UB:
///
/// * change of positions of priorly pushed elements,
/// * removal of elements from the vector.
///
/// Therefore these methods are marked as **unsafe**.
///
/// # Safety - [ImpVec](https://crates.io/crates/orx-imp-vec)
///
/// `PinnedVec` is in the core of the types which aim to make defining
/// relational data structures safe and convenient.
/// This trait defines required methods which `ImpVec` requires
/// to provide the safe api to achieve this goal.
///
/// All these methods have default implementations which simply return nothing.
/// Depending on the data structure to be defined,
/// it is sufficient to implement the relevant methods.
///
/// For instance, [LinkedList](https://crates.io/crates/orx-linked-list) implements
/// methods `prev`, `next`, `prev_mut` and `next_mut`,
/// and uses `set_prev` and `set_next` methods of `ImpVec` to safely define/maintain its
/// internal references.
pub trait SelfRefVecItem<'a> {
    /// Reference to the previous element of this element;
    /// None if this element is at the beginning.
    fn prev(&self) -> Option<&'a Self> {
        None
    }
    /// Reference to the next element of this element;
    /// None if this element is at the end.
    fn next(&self) -> Option<&'a Self> {
        None
    }
    /// Mutable reference to the previous element of this element;
    /// None if this element is at the beginning.
    fn set_prev(&mut self, prev: Option<&'a Self>) {}
    /// Mutable reference to the next element of this element;
    /// None if this element is at the end.
    fn set_next(&mut self, next: Option<&'a Self>) {}
}
