use regex::Regex;
use std::sync::LazyLock;

static FIRST_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^What is (-?\d+)(.+)?\?$").unwrap());
static BASIC_OP_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^ (plus|minus|divided by|multiplied by) (-?\d+)(.+)?$").unwrap());
static POWER_OP_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^ (?:raised to the) (\d+)(?:st|nd|rd|th) power(.+)?$").unwrap());

pub fn parse_prefix(command: &str) -> Option<(i32, Option<&str>)> {
    let tokens = FIRST_RE.captures(command)?;
    let number = tokens.get(1)?.as_str().parse::<i32>().ok()?;
    let remaining = tokens.get(2).and_then(|m| m.as_str().into());
    Some((number, remaining))
}

pub fn parse_basic_op(expression: &str, left_value: i32) -> Option<(i32, Option<&str>)> {
    let tokens = BASIC_OP_RE.captures(expression)?;
    let op = tokens.get(1)?.as_str();
    let mut result = left_value;
    let number = tokens.get(2)?.as_str().parse::<i32>().ok()?;
    match op {
        "plus" => result += number,
        "minus" => result -= number,
        "multiplied by" => result *= number,
        "divided by" => result /= number,
        _ => return None,
    }
    let remaining = tokens.get(3).and_then(|m| m.as_str().into());
    Some((result, remaining))
}

pub fn parse_power_op(expression: &str, base: i32) -> Option<(i32, Option<&str>)> {
    let tokens = POWER_OP_RE.captures(expression)?;
    let exponent = tokens.get(1)?.as_str().parse::<i32>().ok()?;
    let remaining = tokens.get(2).and_then(|m| m.as_str().into());
    let result = base.pow(exponent as u32);
    Some((result, remaining))
}

pub fn answer(command: &str) -> Option<i32> {
    let mut state = parse_prefix(command);
    while let Some((number, expression)) = state {
        if let Some(expr) = expression {
            state = parse_basic_op(expr, number).or_else(|| parse_power_op(expr, number));
        } else {
            break;
        }
    }
    Some(state?.0)
}
