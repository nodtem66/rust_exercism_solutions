#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum WordyOperation {
    Plus,
    Minus,
    Multiplied,
    Divided,
    Exponentiated,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct WordyStack {
    left_value: Option<i32>,
    operator: Option<WordyOperation>,
    right_value: Option<i32>,
}

impl WordyStack {
    fn new() -> Self {
        WordyStack {
            left_value: None,
            operator: None,
            right_value: None,
        }
    }



    fn compute(&self) -> Option<i32> {
        let left = self.left_value?;
        let only_left = self.operator.is_none() && self.right_value.is_none();
        if only_left {
            return Some(left);
        };
        let right = self.right_value?;
        match self.operator? {
            WordyOperation::Plus => Some(left + right),
            WordyOperation::Minus => Some(left - right),
            WordyOperation::Multiplied => Some(left * right),
            WordyOperation::Divided => Some(left / right),
            WordyOperation::Exponentiated => Some(left.pow(right as u32)),
        }
    }

    fn compute_and_move_to_left(&mut self) {
        if let Some(result) = self.compute() {
            self.left_value = Some(result);
            self.operator = None;
            self.right_value = None;
        }
    }
}

fn parse(math_expression: Option<&str>) -> Option<i32> {
    let expression = math_expression?;

    let tokens: Vec<&str> = expression.split_whitespace().collect();
    if tokens.is_empty() {
        return None;
    }
    let mut result = WordyStack::new();
    for token in tokens {
        match token {
            "plus" | "minus" | "multiplied" | "divided" | "exponentiated" => {
                if result.left_value.is_none() || (result.operator.is_some() && result.right_value.is_none()) {
                    return None;
                }
                if result.operator.is_some() && result.right_value.is_some() {
                    result.compute_and_move_to_left();
                }
                result.operator = match token {
                    "plus" => Some(WordyOperation::Plus),
                    "minus" => Some(WordyOperation::Minus),
                    "multiplied" => Some(WordyOperation::Multiplied),
                    "divided" => Some(WordyOperation::Divided),
                    "exponentiated" => Some(WordyOperation::Exponentiated),
                    _ => None,
                };
            },
            _ => {
                println!("Token: {}", token);
                match token.parse::<i32>() {
                    Ok(num) => {
                        println!("Parsed number: {}", num);
                        if result.left_value.is_none() {
                            result.left_value = Some(num);
                        } else if result.operator.is_some() && result.right_value.is_none() {
                            result.right_value = Some(num);
                        } else {
                            return None;
                        }
                    }
                    Err(_) => {
                        if token != "by" && token != "raised" {
                            return None;
                        }
                    }
                }
            }
        }
    }

    result.compute()
}

pub fn answer(command: &str) -> Option<i32> {

    if command.get(0..7) != Some("What is") {
        return None;
    }

    let n = command.len();

    if command.get(n - 1..n) != Some("?") {
        return None;
    }

    return parse(command.get(7..n - 1));
}
