use std::iter;
pub fn egg_count(display_value: u32) -> usize {
    iter::successors(Some(display_value), |&n| Some(n >> 1))
        .take_while(|&n| n > 0)
        .map(|n| n & 1)
        .sum::<u32>() as usize
}
