use lazy_regex::*;

static FIRST_RE: Lazy<Regex> = lazy_regex!(r"^What is (-?\d+)(.+)?\?$");
static BASIC_OP_RE: Lazy<Regex> =
    lazy_regex!(r"^ (plus|minus|divided by|multiplied by) (-?\d+)(.+)?$");
static POWER_OP_RE: Lazy<Regex> =
    lazy_regex!(r"^ (?:raised to the) (\d+)(?:st|nd|rd|th) power(.+)?$");

pub fn parse_prefix(command: &str) -> Option<(i32, Option<&str>)> {
    let tokens = FIRST_RE.captures(command)?;
    let number = tokens.get(1)?.as_str().parse::<i32>().ok()?;
    let remaining = tokens.get(2).and_then(|m| m.as_str().into());
    println!("Parsed prefix: number = {}, remaining = {:?}", number, remaining);
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
    println!("Operation: {}, number: {}, result: {}", op, number, result);
    Some((result, remaining))
}

pub fn parse_power_op(expression: &str, base: i32) -> Option<(i32, Option<&str>)> {
    let tokens = POWER_OP_RE.captures(expression)?;
    let exponent = tokens.get(1)?.as_str().parse::<i32>().ok()?;
    let remaining = tokens.get(2).and_then(|m| m.as_str().into());
    let result = base.pow(exponent as u32);
    println!("base: {}, exponent: {}, result: {}", base, exponent, result);
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
