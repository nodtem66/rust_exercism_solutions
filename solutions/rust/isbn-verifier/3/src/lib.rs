/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let x: Vec<u32> = isbn
        .chars()
        .filter(|c| *c != '-')
        .enumerate()
        .filter_map(|e| match e {
            (9, 'X') => Some(10),
            (_, '0'..='9') => e.1.to_digit(10),
            _ => Some(11),
        }).collect();
    if x.iter().all(|&d| d < 11) && x.len() == 10 {
        x.iter().rev()
            .enumerate()
            .map(|(i, &d)| d * ((i+1) as u32))
            .sum::<u32>() % 11 == 0
    } else {
        false
    }
}
