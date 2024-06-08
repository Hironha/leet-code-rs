pub fn roman_to_int(roman: String) -> i32 {
    let mut value = 0i32;
    let mut prev = 0i32;

    // iterating in reverse order is easier since it requires less logic
    for ch in roman.chars().rev() {
        let curr = match ch {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };

        if curr >= prev {
            value += curr;
        } else {
            value -= curr;
        }

        prev = curr;
    }

    value
}

#[cfg(test)]
mod tests {
    use super::roman_to_int;

    #[test]
    fn case1() {
        assert_eq!(roman_to_int(String::from("III")), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(roman_to_int(String::from("LVIII")), 58);
    }

    #[test]
    fn case3() {
        assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
    }
}
