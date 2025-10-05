/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let x: Vec<u32> = isbn
        .chars()
        .filter(|c| *c != '-')
        .enumerate()
        .filter_map(|e| match e {
            (9, 'X') => Some(10),
            (_, c) => c.to_digit(10),
        }).collect();
    if x.len() == 10 {
        x.iter().rev()
            .enumerate()
            .map(|(i, &d)| d * (i as u32))
            .sum::<u32>() % 11 == 0
    } else {
        false
    }
}
