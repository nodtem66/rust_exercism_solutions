/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut sum = 0;
    let mut multiplier = 10;

    for c in isbn.chars() {
        match c {
            '0'..='9' => {
                if multiplier == 0 {
                    return false;
                }
                sum += (c as u32 - '0' as u32) * multiplier;
                multiplier -= 1;
            }
            'X' if multiplier == 1 => {
                sum += 10;
                multiplier -= 1;
            }
            '-' => continue,
            _ => return false,
        }
    }

    sum % 11 == 0 && multiplier == 0
}
