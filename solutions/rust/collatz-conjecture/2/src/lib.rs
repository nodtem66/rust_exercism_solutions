use std::iter;

pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        _ => Some(iter::successors(Some(n), |&x| {
            // iter::successors takes an Option and loop until None,
            // so we return None when we reach 1
            if x == 1 {
                None
            } else if x.is_multiple_of(2) {
                Some(x / 2)
            } else {
                Some(3 * x + 1)
            }
        }).count() as u64 - 1),
    }
}
