fn get_pair(symbol: &char) -> Option<char> {
    match symbol {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        _ => None,
    }
}

// the idea is to keep track of open symbols in a stack
// and if the current symbol is a close symbol but no the
// one the latest open symbol requires, then it's not valid
// otherwise, after validating all the input, if the open symbol stack
// is empty, it means it's valid
pub fn validate_parenthesis(input: &str) -> bool {
    let mut open_stack: Vec<char> = Vec::new();

    for symbol in input.chars() {
        let open_symbol = open_stack.last();

        match open_symbol {
            Some(open_symbol) => {
                let must_close_symbol = get_pair(open_symbol).unwrap();
                if must_close_symbol == symbol {
                    open_stack.pop();
                } else if get_pair(&symbol).is_some() {
                    open_stack.push(symbol);
                } else {
                    return false;
                }
            }
            None => {
                if get_pair(&symbol).is_some() {
                    open_stack.push(symbol);
                } else {
                    return false;
                }
            }
        };
    }

    open_stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_valid() {
        let input_1 = "()".to_owned();
        let input_2 = "[]".to_owned();
        let input_3 = "{}".to_owned();
        let input_4 = "(){}".to_owned();
        let input_5 = "()[]{}".to_owned();
        let input_6 = "()()[]{}".to_owned();

        assert!(validate_parenthesis(&input_1));
        assert!(validate_parenthesis(&input_2));
        assert!(validate_parenthesis(&input_3));
        assert!(validate_parenthesis(&input_4));
        assert!(validate_parenthesis(&input_5));
        assert!(validate_parenthesis(&input_6));
    }

    #[test]
    fn should_be_invalid() {
        let input_1 = "(()".to_owned();
        let input_2 = "([)]".to_owned();
        let input_3 = "(}){}".to_owned();

        assert!(!validate_parenthesis(&input_1));
        assert!(!validate_parenthesis(&input_2));
        assert!(!validate_parenthesis(&input_3));
    }
}
