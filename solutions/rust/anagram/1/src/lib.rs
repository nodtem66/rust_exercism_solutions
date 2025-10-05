use std::collections::HashSet;
use itertools::sorted;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    let word_sorted = sorted(word.to_lowercase().chars()).collect::<String>();
    for &candidate in possible_anagrams {
        let candidate_sorted = sorted(candidate.to_lowercase().chars()).collect::<String>();
        let not_anagram_of_themselves = word.to_lowercase() != candidate.to_lowercase();
        let is_anagram = candidate_sorted == word_sorted;
        if is_anagram && not_anagram_of_themselves {
            result.insert(candidate);
        }
    }
    result
}
