mod roman {
    fn roman_char_to_int(roman_char: &char) -> Option<i32> {
        match roman_char {
            'I' => Some(1),
            'V' => Some(5),
            'X' => Some(10),
            'L' => Some(50),
            'C' => Some(100),
            'D' => Some(500),
            'M' => Some(1000),
            _ => None,
        }
    }

    pub fn roman_to_int(roman: String) -> i32 {
        let mut sum: i32 = 0;
        let mut previous: i32 = 0;

        for roman_char in roman.chars().rev() {
            if let Some(value) = roman_char_to_int(&roman_char) {
                if value < previous {
                    sum -= value;
                } else {
                    sum += value;
                }
                previous = value;
            }
        }

        sum
    }
}

#[cfg(test)]
mod roman_test {
    use super::roman;

    #[test]
    fn should_equal() {
        let r_13 = String::from("XIII");
        assert_eq!(roman::roman_to_int(r_13), 13);

        let r_1234 = String::from("MCCXXXIV");
        assert_eq!(roman::roman_to_int(r_1234), 1234);
    }
}

fn main() {
    let roman = String::from("XIII");

    assert_eq!(roman::roman_to_int(roman), 13);
}
