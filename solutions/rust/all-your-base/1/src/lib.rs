use std::iter;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}
///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    if number.is_empty() {
        return Ok(vec![0]);
    }
    let mut base10: u32 = 0;
    for (i, &digit) in number.iter().rev().enumerate() {
        if digit >= from_base {
            return Err(Error::InvalidDigit(digit));
        }
        base10 += digit * from_base.pow(i.try_into().unwrap());
    }
    let l: Vec<(u32, u32)> = iter::successors(Some((base10 / to_base, base10 % to_base)), |&n| {
        if n.0 == 0 {
            None
        } else {
            Some((n.0 / to_base, n.0 % to_base))
        }
    }).collect();
    Ok(l.iter().rev().map(|&(_, y)| y).collect())
}
