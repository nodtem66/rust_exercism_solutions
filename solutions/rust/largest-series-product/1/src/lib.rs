#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

fn product_of_digits(digits: &str) -> u64 {
    digits
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .product()
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if let Some(c) = string_digits.chars().find(|c| !c.is_ascii_digit()) {
        return Err(Error::InvalidDigit(c));
    }
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    let n = string_digits.len() - span + 1;
    Ok((0..n).map(|i| product_of_digits(&string_digits[i..i + span])).max().unwrap())
}
