pub fn abbreviate(phrase: &str) -> String {
    // I tried to make it work with a single iterator chain, but it got too messy.
    let mut result = String::new();
    for word in phrase.split(&[' ', '-']) {
        for (i, c) in word.chars().filter(|c| c.is_alphabetic()).enumerate() {
            if i == 0 {
                result.push(c.to_ascii_uppercase());
            } else if c.is_uppercase() && !word.chars().nth(i - 1).unwrap().is_uppercase() {
                result.push(c);
            }
        }
    }
    result
}
