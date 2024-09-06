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
    if slice.is_empty() {
        false
    } else {
        let ptr_element = element as *const T as usize;
        let ptr = slice.as_ptr();
        let ptr_beg = ptr as usize;
        if ptr_element < ptr_beg {
            false
        } else {
            let ptr_end = (unsafe { ptr.add(slice.len() - 1) }) as usize;
            ptr_element <= ptr_end
        }
    }
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
