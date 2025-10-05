pub fn check(candidate: &str) -> bool {
    let word: String = candidate.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect();
    word.len() == word.chars().collect::<std::collections::HashSet<char>>().len()
}
