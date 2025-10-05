pub fn abbreviate(phrase: &str) -> String {
    phrase.split(&[' ', '-']).map(find_acronym_from_word).collect::<String>()
}

fn find_acronym_from_word(word: &str) -> String {
    if word.chars().all(|c| c.is_uppercase()) || word.chars().all(|c| c.is_lowercase()) {
        match word.chars().next() {
            Some(c) => c.to_ascii_uppercase().to_string(),
            _ => String::new(),
        }
    } else {
        word.chars()
            .filter(|c| c.is_uppercase())
            .collect::<String>()
    }
}
