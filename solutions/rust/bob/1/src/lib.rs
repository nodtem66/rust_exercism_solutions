pub fn reply(message: &str) -> &str {
    let is_question = message.trim().ends_with("?");
    let is_all_uppercase =message.trim().chars().filter(|c| c.is_ascii_alphabetic()).all(|c| c.is_uppercase());
    let has_letters = message.chars().any(|c| c.is_ascii_alphabetic());
    let is_yelling = has_letters && is_all_uppercase;
    let is_silent = message.trim().chars().filter(|c| c.is_ascii_alphanumeric()).count() == 0;
    if is_question && is_yelling {
        return "Calm down, I know what I'm doing!";
    }
    if is_question {
        return "Sure.";
    }
    if is_silent {
        return "Fine. Be that way!";
    }
    if is_yelling {
        return "Whoa, chill out!";
    }
    "Whatever."
}
