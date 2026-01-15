pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = vec![];
    let mut m = 2;
    let mut reminder = n;
    while reminder > 1 {
        while reminder % m == 0 {
            factors.push(m);
            reminder /= m;
        }
        m += 1;
    }
    factors
}
