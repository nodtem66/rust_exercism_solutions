use itertools::Itertools;

const LATIN_ALPHABET_LEN: u32 = 26;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Calculate Greatest Common Divisor (GCD) of a and b using Stein's algorithm
/// https://en.wikipedia.org/wiki/Binary_GCD_algorithm
/// Returns GCD of a and b
fn gcd(mut a: u32, mut b: u32) -> u32 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    // gcd(2ⁱ a, 2ʲ b) = 2ᵏ gcd(a, b) with a, b odd and k = min(i, j)
    let k = (a | b).trailing_zeros(); // common factors of 2ᵏ

    // gcd(2ⁱ a, b) = gcd(a, b) if b is odd
    a >>= a.trailing_zeros();

    while b > 0 {
        // gcd(a, 2ʲ b) = gcd(a, b) if a is odd
        b >>= b.trailing_zeros();
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        // Now a ≤ b and both a and b are odd
        // gcd(a, b) = gcd(a, b - a)
        b -= a;
        // gcd(a, 2ʲ b) = gcd(a, b) if a is odd
    }
    a << k
}

fn mmi(a: i32, m: i32) -> i32 {
    // extedned Euclidean algorithm to find modular multiplicative inverse
    // extedned_gcd(a, m)
    let (mut prev_r, mut r) = (a, m);
    if a < m {
        std::mem::swap(&mut prev_r, &mut r);
    }
    let (mut prev_s, mut s) = (1i32, 0i32);

    while r != 0 {
        let q = prev_r / r;
        (prev_r, r) = (r, prev_r - q * r);
        (prev_s, s) = (s, prev_s - q * s);
    }

    if a < m {
        (prev_r - prev_s * m) / a
    } else {
        prev_s
    }
}

fn is_coprime(a: u32, b: u32) -> bool {
    gcd(a, b) == 1
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !is_coprime(a as u32, LATIN_ALPHABET_LEN) {
        return Err(AffineCipherError::NotCoprime(a));
    }
    let mut encoded = Vec::new();
    for c in plaintext.chars().filter(|c| c.is_ascii_alphanumeric()) {
        if c.is_ascii_alphabetic() {
            let x = c.to_ascii_lowercase() as u8 - b'a';
            let encoded_char = ((a * x as i32 + b) % LATIN_ALPHABET_LEN as i32) as u8;
            let encoded_char = encoded_char + b'a';
            encoded.push(encoded_char);
        } else {
            encoded.push(c as u8);
        }
    }
    // Format output in chunks of 5 characters
    Ok(encoded
        .into_iter()
        .chunks(5)
        .into_iter()
        .filter_map(|chunk| String::from_utf8(chunk.collect()).ok())
        .collect::<Vec<String>>()
        .join(" "))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !is_coprime(a as u32, LATIN_ALPHABET_LEN) {
        return Err(AffineCipherError::NotCoprime(a));
    }
    let mut decoded = Vec::new();
    for c in ciphertext.chars().filter(|c| c.is_ascii_alphanumeric()) {
        if c.is_ascii_alphabetic() {
            let y = c.to_ascii_lowercase() as u8 - b'a';
            let decoded_char =
                (mmi(a, LATIN_ALPHABET_LEN as i32) * (y as i32 - b)) % LATIN_ALPHABET_LEN as i32;
            let decoded_char = if decoded_char < 0 {
                (decoded_char + LATIN_ALPHABET_LEN as i32) as u8
            } else {
                decoded_char as u8
            };
            decoded.push(decoded_char + b'a');
        } else {
            decoded.push(c as u8);
        }
    }
    Ok(String::from_utf8(decoded).unwrap())
}
