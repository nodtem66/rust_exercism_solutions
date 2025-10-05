const NUMBER_WORDS: [&'static str; 11] = [
    "no", "one", "two", "three", "four", "five",
    "six", "seven", "eight", "nine", "ten"
];

fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn pluralize_bottle(count: u32) -> &'static str {
    if count == 1 { "bottle" } else { "bottles" }
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    assert!(start_bottles > 0 && start_bottles <= 10);
    assert!(take_down > 0 && take_down <= start_bottles);
    let mut song = Vec::new();
    let mut bottles = start_bottles;
    for _ in 0..take_down {
        let capitalized = capitalize_first(NUMBER_WORDS[bottles as usize]);
        let bottle_word = pluralize_bottle(bottles);
        let next_bottles = NUMBER_WORDS[(bottles - 1) as usize];
        let next_bottle_word = pluralize_bottle(bottles - 1);
        song.push(format!(
            "{capitalized} green {bottle_word} hanging on the wall,\n\
            {capitalized} green {bottle_word} hanging on the wall,\n\
            And if one green bottle should accidentally fall,\n\
            There'll be {next_bottles} green {next_bottle_word} hanging on the wall."
        ));
        bottles -= 1;
    }
    song.join("\n\n")
}
