use std::cmp::Ordering;

pub fn find<T: AsRef<[U]>, U: Ord>(array: T, key: U) -> Option<usize> {
    let mut start = 0;
    let mut end = array.as_ref().len().wrapping_sub(1);

    while start <= end {
        let mid = (start + end) / 2;
        let Some(current) = array.as_ref().get(mid) else {
            break;
        };
        match current.cmp(&key) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => start = mid.wrapping_add(1),
            Ordering::Greater => end = mid.wrapping_sub(1),
        }
    }
    None
}