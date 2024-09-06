use core::ops::{Bound, RangeBounds};

pub(crate) fn range_start<R: RangeBounds<usize>>(range: &R) -> usize {
    match range.start_bound() {
        Bound::Excluded(x) => x + 1,
        Bound::Included(x) => *x,
        Bound::Unbounded => 0,
    }
}
pub(crate) fn range_end<R: RangeBounds<usize>>(range: &R, vec_len: usize) -> usize {
    match range.end_bound() {
        Bound::Excluded(x) => *x,
        Bound::Included(x) => x + 1,
        Bound::Unbounded => vec_len,
    }
}
