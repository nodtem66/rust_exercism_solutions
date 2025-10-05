use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn new(value: u64, min: u64, max: u64) -> Self {
        let low_factors: Vec<u64> = (min..=max)
            .filter(|x| value.is_multiple_of(*x))
            .filter(|x| (value / *x) <= max)
            .take_while(|&x| x * x <= value)
            .collect();
        let mut factors = HashSet::new();
        for &factor in &low_factors {
            factors.insert((factor, value / factor));
        }
        Self { value, factors }
    }
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }

    
}

fn is_palindrome(n: u64, min: u64, max: u64) -> Option<Palindrome> {
    let s = n.to_string();
    let is_palindrome = s.chars().eq(s.chars().rev());
    if is_palindrome {
        let p = Palindrome::new(n, min, max);
        if !p.factors.is_empty() {
            return Some(p);
        }
    }
    None
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let smallest_palindrome = (min*min..=max*max).filter_map(move |x| is_palindrome(x, min, max)).next();
    let largest_palindrome = (min*min..=max*max).rev().filter_map(move |x| is_palindrome(x, min, max)).next();
    match (smallest_palindrome, largest_palindrome) {
        (Some(small), Some(larg)) => Some((small, larg)),
        _ => None,
    }
}
