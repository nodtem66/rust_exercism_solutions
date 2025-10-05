pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut residue = n;
    let mut count: u64 = 0;
    while residue > 1 {
        match residue.is_multiple_of(2) {
            true => residue /= 2,
            false => residue = 3 * residue + 1,
        }
        count += 1;
    }
    Some(count)
}
