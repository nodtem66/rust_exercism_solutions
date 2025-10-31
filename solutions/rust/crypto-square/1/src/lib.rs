pub fn encrypt(input: &str) -> String {
    let input: String = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let n = input.len();
    if n == 0 {
        return String::new();
    }
    let c = (n as f64).sqrt().ceil() as usize;
    let r = match c * (c - 1) >= n {
        true => c - 1,
        false => c,
    };
    let mut encoded = vec![' '; r * c];
    for i in 0..n {
        let row = i / c;
        let col = i % c;
        let j = col * r + row;
        encoded[j] = input.chars().nth(i).unwrap();
    }
    let mut result = Vec::new();
    for c in encoded.chunks(r) {
        let s: String = c.iter().collect();
        result.push(s);
    }
    result.join(" ")
}
