use std::collections::HashMap;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut marker = HashMap::new();
    let mut primes = Vec::new();
    for num in 2..=upper_bound {
        if !marker.contains_key(&num) {
            primes.push(num);
            let mut multiple = num * 2;
            while multiple <= upper_bound {
                marker.insert(multiple, true);
                multiple += num;
            }
        }
    }
    primes
}
