pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    if digits.iter().all(|d| *d == 9) {
        digits.push(0);
        let last_idx = digits.len() - 1;
        digits.swap(0, last_idx);
    }

    let mut carry = 1i32;
    let mut idx = digits.len();

    while idx > 0 && carry > 0 {
        let i = idx - 1;
        match digits[i] + carry {
            10 => digits[i] = 0,
            sum => {
                digits[i] = sum;
                carry = 0
            }
        };

        idx -= 1;
    }

    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let digits = vec![1, 2, 3];

        assert_eq!(plus_one(digits), vec![1, 2, 4]);
    }

    #[test]
    fn case2() {
        let digits = vec![4, 3, 2, 1];

        assert_eq!(plus_one(digits), vec![4, 3, 2, 2]);
    }

    #[test]
    fn case3() {
        let digits = vec![9];

        assert_eq!(plus_one(digits), vec![1, 0]);
    }
}
