pub fn series(digits: &str, len: usize) -> Vec<String> {
    let d: Vec<char> = digits.chars().collect();
    d.windows(len).map(|w| w.iter().collect::<String>()).collect::<Vec<String>>()
}
