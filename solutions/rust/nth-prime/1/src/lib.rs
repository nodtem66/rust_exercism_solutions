pub fn nth(n: u32) -> u32 {
    // The first prime number is 2, so if n is 0, return 2
    let mut primes = vec!(2, 3);
    // The current_index tracks how many primes we have found so far
    let mut current_index = 1;
    // We will keep finding primes until we reach the nth prime
    while current_index < n {
        // Start checking for the next prime from the last found prime + 2
        // Yes, plus 2 since even numbers > 2 are not prime
        let mut next_prime: u32 = primes.last().unwrap() + 2; 
        // Keep incrementing next_prime by 2 until the number is not divisible by any known primes
        while primes.iter().any(|&p| next_prime.is_multiple_of(p)) {
            next_prime += 2;
        }
        // We found a new prime, add it to the list and increment the index
        primes.push(next_prime);
        current_index += 1;
    }
    primes[n as usize]
}
