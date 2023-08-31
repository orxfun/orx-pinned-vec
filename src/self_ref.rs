use std::num::*;

/// Marker trait for types to be contained in a `PinnedVec` which do not hold a field
/// which is a reference to a another element of the same vector.
///
/// Note that any type which does not implement `NotSelfRefVecItem` marker trait
/// is accepted to be a self-referencing-vector-item due to the following:
///
/// * `PinnedVec` is particularly useful for defining complex data structures
/// which elements of which often references each other;
/// in other words, most of the time `PinnedVec<T>` will be used.
/// * To be able to use `PinnedVecSimple: PinnedVec` for safe calls to `insert`, `remove`, `pop` and `clone`,
/// one must explicitly implement `NotSelfRefVecItem` for the elements
/// explicitly stating the safety of the usage.
/// * Finally, this trait is already implemented for most of the common
/// primitive types.
///
/// It is more brief to describe what a self-referencing-vector-item is
/// rather than describing `NotSelfRefVecItem`.
/// Therefore, such an example struct is demonstrated in the following section.
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
/// Further assume that we want to keep nodes of all trees in the same vector.
/// Compared to alternatives, this is helpful at least for the following reasons:
///
/// * keeping all nodes together helps in achieving better cache locality,
/// * the references defining the tree structure are thin rather than wide pointers,
/// * requires less heap allocations: only the vector is allocated together with all its elements,
/// as opposed to allocating each node separately in an arbitrary memory location.
///
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
/// This is not an undefined behavior (UB) in the classical sense; however,
/// it is certainly an UB in terms of the tree that the vector is supposed to represent.
///
/// Alternatively, if we call `remove(1)` on this vector, we end up with the vector `[ x ]`.
/// Now `x` is pointing to a memory location that does not belong to this vector;
/// we end up with a classical UB this time.
///
/// Therefore, all mut methods which change positions of already existing elements
/// (including removal of elements from the vector)
/// are considered to be **`unsafe`** when `T` is not a `NotSelfRefVecItem`.
///
/// # Significance
///
/// ## Unsafe methods when `T` is not a `NotSelfRefVecItem`
///
/// Whether the element type of a `PinnedVec` is `NotSelfRefVecItem` or not
/// has an impact on the mutable operations which change positions of already pushed elements.
///
/// The following is the complete list of these methods:
///
/// * `insert`
/// * `remove`
/// * `pop`
/// * `clone`
///
/// These methods can be called safely when `T: NotSelfRefVecItem`;
/// only within an `unsafe` block otherwise.
///
/// ## Safe methods regardless of `T` is a `NotSelfRefVecItem` or not
///
/// On the other hand, `mut` methods such as `push` or `extend_from_slice` are **safe**
/// since a `PinnedVec` keeps positions of already pushed elements intact while growing.
/// Therefore, references to already pushed elements will remain valid.
///
/// Although it leads to removal of elements,
/// `clear` is also **safe** for all element types.
/// This is because all elements are dropped together with their possible references.
///
/// # Trait Implementations
///
/// To pick the conservative implementation,
/// one must explicitly implement this marker trait `NotSelfReferencingVecItem`
/// on the type to be contained in a `PinnedVec`.
///
/// On the other hand, they are already implemented for primitives for convenience
/// such as numeric types, strings, etc.
pub trait NotSelfRefVecItem {}

// auto impl
impl<T> NotSelfRefVecItem for &T where T: NotSelfRefVecItem {}
impl<T> NotSelfRefVecItem for Option<T> where T: NotSelfRefVecItem {}

impl NotSelfRefVecItem for bool {}
impl NotSelfRefVecItem for char {}
impl NotSelfRefVecItem for f32 {}
impl NotSelfRefVecItem for f64 {}
impl NotSelfRefVecItem for i128 {}
impl NotSelfRefVecItem for i16 {}
impl NotSelfRefVecItem for i32 {}
impl NotSelfRefVecItem for i64 {}
impl NotSelfRefVecItem for i8 {}
impl NotSelfRefVecItem for isize {}
impl NotSelfRefVecItem for NonZeroI128 {}
impl NotSelfRefVecItem for NonZeroI16 {}
impl NotSelfRefVecItem for NonZeroI32 {}
impl NotSelfRefVecItem for NonZeroI64 {}
impl NotSelfRefVecItem for NonZeroI8 {}
impl NotSelfRefVecItem for NonZeroIsize {}
impl NotSelfRefVecItem for NonZeroU128 {}
impl NotSelfRefVecItem for NonZeroU16 {}
impl NotSelfRefVecItem for NonZeroU32 {}
impl NotSelfRefVecItem for NonZeroU64 {}
impl NotSelfRefVecItem for NonZeroU8 {}
impl NotSelfRefVecItem for NonZeroUsize {}
impl NotSelfRefVecItem for str {}
impl NotSelfRefVecItem for String {}
impl NotSelfRefVecItem for u128 {}
impl NotSelfRefVecItem for u16 {}
impl NotSelfRefVecItem for u32 {}
impl NotSelfRefVecItem for u64 {}
impl NotSelfRefVecItem for u8 {}
