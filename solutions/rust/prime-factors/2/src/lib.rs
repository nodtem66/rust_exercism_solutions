pub fn factors(mut n: u64) -> Vec<u64> {
    // let mut factors: Vec<u64> = Vec::new();
    // // The first prime number is 2, so if n is 0, return 2
    // let mut primes = vec!(2, 3);
    // let mut last_prime = 3;
    // while last_prime * last_prime <= n {
    //     // Start checking for the next prime from the last found prime + 2
    //     // Yes, plus 2 since even numbers > 2 are not prime
    //     let mut next_prime = last_prime + 2; 
    //     // Keep incrementing next_prime by 2 until the number is not divisible by any known primes
    //     while primes.iter().any(|&p| next_prime.is_multiple_of(p)) {
    //         next_prime += 2;
    //     }
    //     // We found a new prime, add it to the list and increment the index
    //     primes.push(next_prime);
    //     last_prime = next_prime;
    // }

    // let mut remainder = n;
    // for prime in primes {
    //     while remainder.is_multiple_of(prime) {
    //         factors.push(prime);
    //         remainder /= prime;
    //     }
    //     if remainder == 1 {
    //         break;
    //     }
    // }
    // if remainder > 1 {
    //     factors.push(remainder);
    // }
    // factors
    let mut factors = vec![];
    let mut m = 2;
    while n > 1 {
        while n % m == 0 {
            factors.push(m);
            n /= m;
        }
        m += 1;
    }
    factors
}
