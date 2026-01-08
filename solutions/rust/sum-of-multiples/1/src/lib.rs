use std::collections::HashSet;
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors.iter()
        .filter(|&&f| f > 0)
        .flat_map(|&f| (1..=(limit-1)/f).map(|i| i*f).collect::<Vec<u32>>())
        .collect::<HashSet<u32>>()
        .into_iter()
        .sum()
}
