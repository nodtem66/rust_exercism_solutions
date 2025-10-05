/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let sum: u32 = code.chars().filter(|c| c.is_digit(10)).rev().enumerate().map(|(i, c)| {
        let n = c.to_digit(10).unwrap();
        match !i.is_multiple_of(2) {
            true => {
                let n = n * 2;
                if n > 9 { n - 9 } else { n }
            }
            false => n,
        }
    }).sum();
    let no_symbols = code.chars().all(|c| c.is_digit(10) || c.is_whitespace());
    let not_single_digit = code.chars().filter(|c| c.is_digit(10)).count() > 1;
    no_symbols && not_single_digit && sum % 10 == 0
}
