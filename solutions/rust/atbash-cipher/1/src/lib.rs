/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut result = String::new();
    for c in plain.chars().filter(|c| !c.is_whitespace()) {
        if c.is_ascii_alphabetic() {
            let x = c.to_ascii_lowercase() as u8 - b'a';
            let atbash_char = b'z' - x;
            result.push(atbash_char.into());
        } else if c.is_ascii_digit() {
            result.push(c);
        }
        if result.len() % 6 == 5 {
            result.push(' ');
        }
    }
    result.trim().to_string()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let mut result = String::new();
    for x in cipher.chars().filter(|c| !c.is_whitespace()) {
        if x.is_ascii_alphabetic() {
            let y = x.to_ascii_lowercase() as u8 - b'a';
            let atbash_char = b'z' - y;
            result.push(atbash_char.into());
        } else if x.is_ascii_digit() {
            result.push(x);
        }
    }
    result
}
