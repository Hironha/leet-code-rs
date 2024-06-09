pub fn is_valid(input: String) -> bool {
    let mut open_stack = Vec::<u8>::new();

    for byte in input.as_bytes() {
        match byte {
            b'{' | b'[' | b'(' => open_stack.push(*byte),
            b'}' => {
                match open_stack.last() {
                    Some(b'{') => open_stack.pop(),
                    Some(_) | None => return false,
                };
            }
            b']' => {
                match open_stack.last() {
                    Some(b'[') => open_stack.pop(),
                    Some(_) | None => return false,
                };
            }
            b')' => {
                match open_stack.last() {
                    Some(b'(') => open_stack.pop(),
                    Some(_) | None => return false,
                };
            }
            _ => return false,
        };
    }

    open_stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::is_valid;

    #[test]
    fn case1() {
        assert!(is_valid(String::from("()")));
    }

    #[test]
    fn case2() {
        assert!(is_valid(String::from("()[]{}")));
    }

    #[test]
    fn case3() {
        assert!(!is_valid(String::from("(]")));
    }
}
