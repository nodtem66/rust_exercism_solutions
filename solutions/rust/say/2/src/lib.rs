pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    let mut residual = n;
    let mut parts = Vec::new();
    let mut exponent = 0;
    // Process each group of three digits
    parts.insert(0, encode_three_digits((residual % 1000) as u16));
    residual /= 1_000;
    exponent += 3;
    while residual > 0 {
        if !residual.is_multiple_of(1000) {
            match exponent {
                3 => parts.insert(0, "thousand".to_string()),
                6 => parts.insert(0, "million".to_string()),
                9 => parts.insert(0, "billion".to_string()),
                12 => parts.insert(0, "trillion".to_string()),
                15 => parts.insert(0, "quadrillion".to_string()),
                18 => parts.insert(0, "quintillion".to_string()),
                _ => {}
            }
        }
        parts.insert(0, encode_three_digits((residual % 1000) as u16));
        residual /= 1_000;
        exponent += 3;
    }
    parts.iter().filter(|s| !s.is_empty()).cloned().collect::<Vec<String>>().join(" ")
}

pub fn encode_three_digits(n: u16) -> String {
    let hundreds = (n / 100) % 10;
    let mut parts = Vec::new();
    if hundreds > 0 {
        parts.push(format!("{} hundred", encode_two_digits(hundreds)));
    }
    parts.push(encode_two_digits(n % 100));
    parts.iter().filter(|s| !s.is_empty()).cloned().collect::<Vec<String>>().join(" ")
}
pub fn encode_two_digits(n: u16) -> String {
    match (n / 10, n % 10) {
        (0, 0) => "".to_string(),
        (0, 1) => "one".to_string(),
        (0, 2) => "two".to_string(),
        (0, 3) => "three".to_string(),
        (0, 4) => "four".to_string(),
        (0, 5) => "five".to_string(),
        (0, 6) => "six".to_string(),
        (0, 7) => "seven".to_string(),
        (0, 8) => "eight".to_string(),
        (0, 9) => "nine".to_string(),
        (1, 0) => "ten".to_string(),
        (1, 1) => "eleven".to_string(),
        (1, 2) => "twelve".to_string(),
        (1, 3) => "thirteen".to_string(),
        (1, 4) => "fourteen".to_string(),
        (1, 5) => "fifteen".to_string(),
        (1, 6) => "sixteen".to_string(),
        (1, 7) => "seventeen".to_string(),
        (1, 8) => "eighteen".to_string(),
        (1, 9) => "nineteen".to_string(),
        (2, 0) => "twenty".to_string(),
        (2, d) => format!("twenty-{}", encode_two_digits(d)),
        (3, 0) => "thirty".to_string(),
        (3, d) => format!("thirty-{}", encode_two_digits(d)),
        (4, 0) => "forty".to_string(),
        (4, d) => format!("forty-{}", encode_two_digits(d)),
        (5, 0) => "fifty".to_string(),
        (5, d) => format!("fifty-{}", encode_two_digits(d)),
        (6, 0) => "sixty".to_string(),
        (6, d) => format!("sixty-{}", encode_two_digits(d)),
        (7, 0) => "seventy".to_string(),
        (7, d) => format!("seventy-{}", encode_two_digits(d)),
        (8, 0) => "eighty".to_string(),
        (8, d) => format!("eighty-{}", encode_two_digits(d)),
        (9, 0) => "ninety".to_string(),
        (9, d) => format!("ninety-{}", encode_two_digits(d)),
        (0, d) => encode_two_digits(d),
        _ => "".to_string(), // This case should not occur for valid two-digit numbers
    }
}
