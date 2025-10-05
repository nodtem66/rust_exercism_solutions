pub struct PascalsTriangle {
    row_count: u32,
    pub factorials: Vec<u32>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        // Precompute factorials 0! to row_count!
        let factorials =
            std::iter::successors(Some((0u32, 1u32)), |&n| Some((n.0 + 1, n.1 * (n.0 + 1))))
                .take(row_count as usize)
                .map(|(_, fact)| fact)
                .collect();
        PascalsTriangle {
            row_count,
            factorials,
        }
    }

    pub fn compute_ncr(&self, n: u32, r: u32) -> u32 {
        match (n, r) {
            (0, _) => 1,
            (_, 0) => 1,
            (n, r) if r > n => 0,
            (n, r) => {
                // Safety: factorials vector is precomputed up to row_count!
                assert!(n < self.factorials.len() as u32 && r <= n);
                self.factorials[n as usize]
                    / (self.factorials[r as usize] * self.factorials[(n - r) as usize])
            }
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.row_count)
            .map(|n| (0..=n).map(|r| self.compute_ncr(n, r)).collect())
            .collect()
    }
}
