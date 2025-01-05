pub fn i32_to_roman(num: i32) -> String {
    let mut roman = String::new();
    let (mut reversed, mut count) = reverse_i32(num);
    while reversed > 0 && count > 0 {
        let digit = reversed % 10;
        if count == 4 {
            convert_thousand(&mut roman, digit);
        } else if count == 3 {
            convert_hundred(&mut roman, digit);
        } else if count == 2 {
            convert_tenth(&mut roman, digit);
        } else if count == 1 {
            convert_unit(&mut roman, digit);
        }

        reversed /= 10;
        count -= 1;
    }

    roman
}

fn convert_thousand(buf: &mut String, digit: i32) {
    for _ in 0..digit {
        buf.push('M');
    }
}

fn convert_hundred(buf: &mut String, digit: i32) {
    if digit == 9 {
        buf.push_str("CM");
    } else if digit == 5 {
        buf.push('D');
    } else if digit == 4 {
        buf.push_str("CD");
    } else if digit > 5 {
        buf.push('D');
        for _ in 0..digit - 5 {
            buf.push('C');
        }
    } else {
        for _ in 0..digit {
            buf.push('C');
        }
    }
}

fn convert_tenth(buf: &mut String, digit: i32) {
    if digit == 9 {
        buf.push_str("XC");
    } else if digit == 5 {
        buf.push('L');
    } else if digit == 4 {
        buf.push_str("XL");
    } else if digit > 5 {
        buf.push('L');
        for _ in 0..digit - 5 {
            buf.push('X');
        }
    } else {
        for _ in 0..digit {
            buf.push('X');
        }
    }
}

fn convert_unit(buf: &mut String, digit: i32) {
    if digit == 9 {
        buf.push_str("IX");
    } else if digit == 5 {
        buf.push('V');
    } else if digit == 4 {
        buf.push_str("IV");
    } else if digit > 5 {
        buf.push('V');
        for _ in 0..digit - 5 {
            buf.push('I');
        }
    } else {
        for _ in 0..digit {
            buf.push('I');
        }
    }
}

fn reverse_i32(mut num: i32) -> (i32, u32) {
    let mut count = 0u32;
    let mut reversed = 0i32;
    while num > 0 {
        reversed = reversed * 10 + num % 10;
        num /= 10;
        count += 1;
    }

    (reversed, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = 3749;
        assert_eq!(i32_to_roman(input), String::from("MMMDCCXLIX"));
    }

    #[test]
    fn case2() {
        let input = 58;
        assert_eq!(i32_to_roman(input), String::from("LVIII"));
    }

    #[test]
    fn case3() {
        let input = 1994;
        assert_eq!(i32_to_roman(input), String::from("MCMXCIV"));
    }
}
