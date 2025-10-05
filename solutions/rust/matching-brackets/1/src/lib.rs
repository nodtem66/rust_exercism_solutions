pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars() {
        match c {
            '(' | '{' | '[' => {
                stack.push(c);
            }
            ')' | '}' | ']' => {
                if let Some(last) = stack.last() {
                    if (last == &'(' && c == ')')
                        || (last == &'{' && c == '}')
                        || (last == &'[' && c == ']')
                    {
                        stack.pop();
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}
