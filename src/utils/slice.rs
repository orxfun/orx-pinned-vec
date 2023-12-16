/// Returns the index of the `element` with the given reference.
/// This method has *O(1)* time complexity.
///
/// # Safety
///
/// The underlying memory of the slice `&[T]` stays pinned as long as
/// the reference is in scope; i.e., is not carried to different memory locations.
///
/// Therefore, it is possible and safe to compare an element's reference
/// to find its position in the vector.
///
/// Out of bounds checks are in place.
pub fn index_of<T>(slice: &[T], element: &T) -> Option<usize> {
    let ptr_element = element as *const T as usize;
    let ptr = slice.as_ptr();
    let ptr_beg = ptr as usize;
    if ptr_element < ptr_beg {
        None
    } else {
        let ptr_end = (unsafe { ptr.add(slice.len() - 1) }) as usize;
        if ptr_element > ptr_end {
            None
        } else {
            let diff = ptr_element - ptr_beg;
            let count = diff / std::mem::size_of::<T>();
            Some(count)
        }
    }
}
