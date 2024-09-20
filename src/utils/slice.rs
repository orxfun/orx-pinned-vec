use core::ops::RangeBounds;

/// Returns the index of the `element` with the given reference inside the `slice`.
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
#[inline(always)]
pub fn index_of<T>(slice: &[T], element: &T) -> Option<usize> {
    index_of_ptr(slice, element as *const T)
}

/// Returns the index of the `element` with the given reference inside the `slice`.
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
pub fn index_of_ptr<T>(slice: &[T], element_ptr: *const T) -> Option<usize> {
    let element_ptr = element_ptr as usize;
    let ptr = slice.as_ptr();
    let ptr_beg = ptr as usize;
    if element_ptr < ptr_beg {
        None
    } else {
        let ptr_end = (unsafe { ptr.add(slice.len() - 1) }) as usize;
        if element_ptr > ptr_end {
            None
        } else {
            let diff = element_ptr - ptr_beg;
            let count = diff / core::mem::size_of::<T>();
            Some(count)
        }
    }
}

/// Returns whether or not `element` with the given reference belongs to the given `slice`.
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
pub fn contains_reference<T>(slice: &[T], element: &T) -> bool {
    contains_ptr(slice, element as *const T)
}

/// Returns whether or not element with the given pointer belongs to the given `slice`.
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
pub fn contains_ptr<T>(slice: &[T], element_ptr: *const T) -> bool {
    if slice.is_empty() {
        false
    } else {
        let ptr_beg = slice.as_ptr();
        if element_ptr < ptr_beg {
            false
        } else {
            let ptr_end = unsafe { ptr_beg.add(slice.len() - 1) };
            element_ptr <= ptr_end
        }
    }
}

/// Returns the inclusive being and exclusive end of the given `range`.
/// The range is bounded by the `vec_len` if it is known and provided.
///
/// # Panics
///
/// Panics if end bound is Unbounded while vec_len is None.
pub fn vec_range_limits<R: RangeBounds<usize>>(range: &R, vec_len: Option<usize>) -> [usize; 2] {
    use core::ops::Bound::*;

    let mut begin = match range.start_bound() {
        Included(&a) => a,
        Excluded(a) => a + 1,
        Unbounded => 0,
    };

    let mut end = match range.end_bound() {
        Excluded(&b) => b,
        Included(b) => b + 1,
        Unbounded => vec_len.expect("Unbounded range without a vec_len"),
    };

    if end < begin {
        end = begin;
    }

    if let Some(len) = vec_len {
        if begin > len {
            begin = len;
        }
        if end > len {
            end = len;
        }
    }

    [begin, end]
}

#[cfg(test)]
mod tests {
    pub use super::*;
    use alloc::vec::Vec;

    #[test]
    fn index_of_wrong() {
        let array1 = [0, 1, 2, 3];
        let array2 = [0, 1, 2];

        let index1 = index_of(&array1, &array2[0]);
        assert!(index1.is_none());

        let index2 = index_of(&array2, &array1[0]);
        assert!(index2.is_none());
    }

    #[test]
    fn index_of_some() {
        let n = 1234;
        let array: Vec<_> = (0..n).collect();

        for i in 0..array.len() {
            let element = &array[i];
            let index = index_of(&array, element);
            assert_eq!(Some(i), index);
        }
    }

    #[test]
    fn contains_reference_wrong() {
        let n = 1234;
        let array1: Vec<_> = (0..n).collect();
        let array2: Vec<_> = (0..(n - 1)).collect();

        for element in array1.iter() {
            assert!(!contains_reference(&array2, element));
        }

        for element in array2.iter() {
            assert!(!contains_reference(&array1, element));
        }
    }

    #[test]
    fn contains_reference_correct() {
        let n = 1111;
        let array: Vec<_> = (0..n).collect();

        for element in array.iter() {
            assert!(contains_reference(&array, element));
        }
    }
}
