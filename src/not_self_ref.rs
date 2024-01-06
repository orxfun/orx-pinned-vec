use std::num::*;

/// Marker trait for types to be contained in a `PinnedVec` which do <ins>not</ins> hold a field
/// which is a reference to a another element of the same vector.
///
/// Note that any type which does not implement `NotSelfRefVecItem` marker trait
/// is assumed to be a self-referencing-vector-item due to the following:
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
/// It is more brief to describe what a `SelfRefVecItem` is
/// rather than describing `NotSelfRefVecItem`.
/// Such data types are particularly useful for data types defining
/// relations about its children such as trees or graphs.
/// An example struct is demonstrated in the following section.
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
/// # Safety
///
/// On the other hand, such data structures require more care about safety and correctness.
/// Since each vector element can hold a reference to another vector element,
/// `mut` methods need to be investigated carefully.
///
/// # Significance
///
/// ## Unsafe methods when `T` is not a `NotSelfRefVecItem`
///
/// Whether the element type of a `PinnedVec` is `NotSelfRefVecItem` or not
/// has an impact on the mutable operations which change positions of already pushed elements.
///
/// The following is the list of these methods:
///
/// * `insert`
/// * `remove`
/// * `pop`
/// * `swap`
/// * `truncate`
/// * `clone`
///
/// These methods can be called safely when `T: NotSelfRefVecItem`;
/// only within an `unsafe` block otherwise.
///
/// # Trait Implementations
///
/// `NotSelfRefVecItem` is a marker trait which is implemented for most primitives; however, one needs to implement
/// for new types to explicitly state that the type is <ins>not</ins> self-referential.
pub trait NotSelfRefVecItem {}

// auto impl
impl<T> NotSelfRefVecItem for &T where T: NotSelfRefVecItem {}
impl<T> NotSelfRefVecItem for Option<T> where T: NotSelfRefVecItem {}
impl<T, E> NotSelfRefVecItem for Result<T, E>
where
    T: NotSelfRefVecItem,
    E: NotSelfRefVecItem,
{
}

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
impl NotSelfRefVecItem for usize {}
