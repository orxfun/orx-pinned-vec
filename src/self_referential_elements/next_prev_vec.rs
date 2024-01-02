/// Trait for elements that holds a vector of references to a collection of next elements.
///
/// This can be considered as a dynamically sized generalization of the [`crate::self_referential_elements::SelfRefNext`] trait.
///
/// A common example is a tree where each node holds a collection of references to the children.
/// Here, we decide to use next as deeper in the tree; however, prev could be used interchangeably.
/// *Chosen convention will be internal to the created self-referential-collection which will expose proper methods such as `children` in this example*.
///
/// Notice that the trait itself has a life time `'a` which is equal to the lifetime of the potential linked elements.
/// This is on purpose as the underlying goal of having these traits it to build self-referential-collections
/// where each element live together and share the same lifetime.
///
/// # Example
///
/// ```rust
/// use orx_pinned_vec::self_referential_elements::*;
///
/// #[derive(PartialEq, Debug)]
/// struct TreeNode<'a, Data> {
///     data: Data,
///     children: Vec<&'a Self>,
///     parent: Option<&'a Self>,
/// }
///
/// impl<'a, Data> SelfRefNextVec<'a, Data> for TreeNode<'a, Data> {
///     fn next_vec(&self) -> &[&'a Self] {
///         &self.children
///     }
///     fn next_vec_mut(&mut self) -> &mut Vec<&'a Self> {
///         &mut self.children
///     }
///     fn set_next_vec(&mut self, next: Vec<&'a Self>) {
///         self.children = next;
///     }
/// }
///
/// impl<'a, Data> SelfRefPrev<'a> for TreeNode<'a, Data> {
///     fn prev(&self) -> Option<&'a Self> {
///         self.parent
///     }
///     fn set_prev(&mut self, prev: Option<&'a Self>) {
///         self.parent = prev;
///     }
/// }
/// ```
///
/// In the following example, we want to be able to build the following tree using our `TreeNode`:
///
/// ```text
/// // 0 --+--a
/// //     |--b -- c
/// ```
///
/// However, as expected, the following code block will not compile, but it is helpful to demonstrate the goal.
///
/// ```rust ignore
/// let new_node = |data| TreeNode {
///     data,
///     children: vec![],
///     parent: None,
/// };
///
/// let mut root = new_node('0');
/// let mut a = new_node('a');
/// let mut b = new_node('b');
/// let mut c = new_node('c');
///
/// root.set_next_vec(vec![&a, &b]);
/// a.set_prev(Some(&root));
/// b.set_prev(Some(&root));
///
/// b.set_next_vec(vec![&c]);
/// c.set_prev(Some(&root));
/// ```
///
/// Although this code block will not compile here, it will be possible to build a tree made out of our `TreeNode`s:
///
/// * We will use an implementation of a `PinnedVec` where the elements are `TreeNode<'a, T>`, such as `orx_split_vec::SplitVec`,
/// * We will wrap our `SplitVec<TreeNode<'_, _>>` within an `orx_imp_vec::ImpVec`,
/// * `ImpVec` will allow us to build the relations safely and conveniently:
///   * using the pinned-element guarantees of the underlying `PinnedVec`, and
///   * using the required referential relations provided by implementing `SelfRefNextVec` and `SelfRefPrev` traits, which represents a common tree node.
/// * The tree can be cheaply toggled between an `ImpVec` and its underlying `PinnedVec` storage to enable / disable mutation.
///
/// Please see [orx_imp_vec](https://crates.io/crates/orx-imp-vec) for details and example implementations.
pub trait SelfRefNextVec<'a, Data> {
    /// Reference to the slice of next elements of this element.
    fn next_vec(&self) -> &[&'a Self];

    /// Returns a mutable reference to the collection of next elements.
    fn next_vec_mut(&mut self) -> &mut Vec<&'a Self>;

    /// Sets next elements of this element to the given `next` collection.
    fn set_next_vec(&mut self, next: Vec<&'a Self>);
}

/// Trait for elements that holds a vector of references to a collection of prev elements.
///
/// This can be considered as a dynamically sized generalization of the [`crate::self_referential_elements::SelfRefPrev`] trait.
///
/// A common example is a tree where each node holds a collection of references to the children.
/// Here, we decide to use prev as deeper in the tree; however, prev could be used interchangeably.
/// *Chosen convention will be internal to the created self-referential-collection which will expose proper methods such as `children` in this example*.
///
/// Notice that the trait itself has a life time `'a` which is equal to the lifetime of the potential linked elements.
/// This is on purpose as the underlying goal of having these traits it to build self-referential-collections
/// where each element live together and share the same lifetime.
///
/// # Example
///
/// ```rust
/// use orx_pinned_vec::self_referential_elements::*;
///
/// #[derive(PartialEq, Debug)]
/// struct TreeNode<'a, Data> {
///     data: Data,
///     children: Vec<&'a Self>,
///     parent: Option<&'a Self>,
/// }
///
/// impl<'a, Data> SelfRefPrevVec<'a, Data> for TreeNode<'a, Data> {
///     fn prev_vec(&self) -> &[&'a Self] {
///         &self.children
///     }
///     fn prev_vec_mut(&mut self) -> &mut Vec<&'a Self> {
///         &mut self.children
///     }
///     fn set_prev_vec(&mut self, prev: Vec<&'a Self>) {
///         self.children = prev;
///     }
/// }
///
/// impl<'a, Data> SelfRefPrev<'a> for TreeNode<'a, Data> {
///     fn prev(&self) -> Option<&'a Self> {
///         self.parent
///     }
///     fn set_prev(&mut self, prev: Option<&'a Self>) {
///         self.parent = prev;
///     }
/// }
/// ```
///
/// In the following example, we want to be able to build the following tree using our `TreeNode`:
///
/// ```text
/// // 0 --+--a
/// //     |--b -- c
/// ```
///
/// However, as expected, the following code block will not compile, but it is helpful to demonstrate the goal.
///
/// ```rust ignore
/// let new_node = |data| TreeNode {
///     data,
///     children: vec![],
///     parent: None,
/// };
///
/// let mut root = new_node('0');
/// let mut a = new_node('a');
/// let mut b = new_node('b');
/// let mut c = new_node('c');
///
/// root.set_prev_vec(vec![&a, &b]);
/// a.set_prev(Some(&root));
/// b.set_prev(Some(&root));
///
/// b.set_prev_vec(vec![&c]);
/// c.set_prev(Some(&root));
/// ```
///
/// Although this code block will not compile here, it will be possible to build a tree made out of our `TreeNode`s:
///
/// * We will use an implementation of a `PinnedVec` where the elements are `TreeNode<'a, T>`, such as `orx_split_vec::SplitVec`,
/// * We will wrap our `SplitVec<TreeNode<'_, _>>` within an `orx_imp_vec::ImpVec`,
/// * `ImpVec` will allow us to build the relations safely and conveniently:
///   * using the pinned-element guarantees of the underlying `PinnedVec`, and
///   * using the required referential relations provided by implementing `SelfRefPrevVec` and `SelfRefPrev` traits, which represents a common tree node.
/// * The tree can be cheaply toggled between an `ImpVec` and its underlying `PinnedVec` storage to enable / disable mutation.
///
/// Please see [orx_imp_vec](https://crates.io/crates/orx-imp-vec) for details and example implementations.
pub trait SelfRefPrevVec<'a, Data> {
    /// Reference to the slice of prev elements of this element.
    fn prev_vec(&self) -> &[&'a Self];

    /// Returns a mutable reference to the collection of prev elements.
    fn prev_vec_mut(&mut self) -> &mut Vec<&'a Self>;

    /// Sets prev elements of this element to the given `prev` collection.
    fn set_prev_vec(&mut self, prev: Vec<&'a Self>);
}
