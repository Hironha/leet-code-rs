pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut carry = true;

    for d in digits.iter().rev() {
        let sum = match carry {
            false => *d,
            true => {
                carry = false;
                *d + 1
            }
        };
        if sum > 9 {
            carry = true;
        }
        result.push(sum % 10);
    }
    if carry {
        result.push(1);
    }

    result.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_success() {
        let input_1: Vec<i32> = vec![1, 2, 3];
        assert_eq!(plus_one(input_1), vec![1, 2, 4]);

        let input_2: Vec<i32> = vec![4, 3, 2, 1];
        assert_eq!(plus_one(input_2), vec![4, 3, 2, 2]);

        let input_3: Vec<i32> = vec![9];
        assert_eq!(plus_one(input_3), vec![1, 0]);

        let input_4: Vec<i32> = vec![9, 9];
        assert_eq!(plus_one(input_4), vec![1, 0, 0]);
    }
}
