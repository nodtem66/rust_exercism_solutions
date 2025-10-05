pub fn encode(source: &str) -> String {
    let mut encoded = String::new();
    let mut count = 0;
    let mut current_char = None;
    for i in source.chars() {
        match current_char {
            Some(c) if c == i => count += 1,
            Some(c) => {
                if count > 1 {
                    encoded.push_str(&format!("{count}{c}"));
                } else {
                    encoded.push(c);
                }
                current_char = Some(i);
                count = 1;
            }
            None => {
                current_char = Some(i);
                count = 1;
            }
        }
    }
    if count > 1 {
        encoded.push_str(&format!("{count}{}",current_char.unwrap()));
    } else if count == 1 {
        encoded.push(current_char.unwrap());
    }
    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut count_str = String::new();
    for i in source.chars() {
        if i.is_ascii_digit() {
            count_str.push(i);
        } else {
            let count = count_str.parse::<usize>().unwrap_or(1);
            decoded.push_str(&i.to_string().repeat(count));
            count_str.clear();
        }
    }
    decoded
}
