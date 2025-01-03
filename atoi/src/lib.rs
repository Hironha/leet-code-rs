enum ParsedI32 {
    Overflow,
    Underflow,
    Value(i32),
}

pub fn atoi(s: String) -> i32 {
    let bytes = s.as_bytes();
    let start = trim_left(bytes);
    if start >= s.len() {
        return 0;
    }

    let (sign, start) = match bytes[start] {
        b'-' => (-1, start + 1),
        b'+' => (1, start + 1),
        _ => (1, start),
    };

    match read_i32(&bytes[start..], sign == 1) {
        ParsedI32::Overflow => i32::MAX,
        ParsedI32::Underflow => i32::MIN,
        ParsedI32::Value(val) => val * sign,
    }
}

fn trim_left(bytes: &[u8]) -> usize {
    let mut i = 0usize;
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }

    i
}

fn read_i32(bytes: &[u8], positive: bool) -> ParsedI32 {
    let mut num = 0i32;
    for ch in bytes {
        if !ch.is_ascii_digit() {
            return ParsedI32::Value(num);
        }

        let digit = i32::from(ch - b'0');
        if let Some(val) = num.checked_mul(10).and_then(|mul| mul.checked_add(digit)) {
            num = val
        } else if positive {
            return ParsedI32::Overflow;
        } else {
            return ParsedI32::Underflow;
        }
    }

    ParsedI32::Value(num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = String::from("42");
        assert_eq!(atoi(input), 42);
    }

    #[test]
    fn case2() {
        let input = String::from("  -042");
        assert_eq!(atoi(input), -42);
    }

    #[test]
    fn case3() {
        let input = String::from("1337c0d3");
        assert_eq!(atoi(input), 1337);
    }

    #[test]
    fn case4() {
        let input = String::from("0-1");
        assert_eq!(atoi(input), 0);
    }

    #[test]
    fn case5() {
        let input = String::from("words and 987");
        assert_eq!(atoi(input), 0);
    }

    #[test]
    fn case6() {
        let input = String::from("-91283472332");
        assert_eq!(atoi(input), -2147483648);
    }

    #[test]
    fn case7() {
        let input = String::from("");
        assert_eq!(atoi(input), 0);
    }
}
