use itertools::Itertools;

const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];

fn is_all_consonants(s: &str) -> bool {
    s.chars().all(|c| !VOWELS.contains(&c))
}

fn translate_word(word: &str) -> String {
    // Rule 1: Words that start with a vowel or "xr" or "yt"
    let rule_1 = word.starts_with(VOWELS) || word.starts_with("xr") || word.starts_with("yt");
    if rule_1 {
        format!("{}ay", word)
    }

    // Rule 3: "qu" after consonant
    else if let Some(i) = word.find("qu") && is_all_consonants(&word[..i]) {
        format!("{}{}ay", &word[i+2..], &word[..i+2])
    }

    // Rule 4: Consonant clusters before "y"
    else if let Some(i) = word.find('y')
        && i > 0
        && is_all_consonants(&word[..i])
    {
        format!("{}{}ay", &word[i..], &word[..i])
    }

    // Rule 2: Consonant clusters at the start
    else {
        let Some(i) = word.find(VOWELS) else {
            // No vowels, treat the whole word as consonants
            return word.to_string();
        };
        format!("{}{}ay", &word[i..], &word[..i])
    }

}

pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(translate_word)
        .collect_vec()
        .join(" ")
}
