pub fn add_binary(a: String, mut b: String) -> String {
    if a.len() < b.len() {
        return add_binary(b, a);
    }
    if b.len() < a.len() {
        b = b.chars().rev().collect::<String>();
        for _ in 0..(a.len() - b.len()) {
            b.push('0');
        }
        return add_binary(a, b.chars().rev().collect());
    }
    
    let mut carry = false;
    let mut sum: Vec<char> = Vec::new();
    for (i, b_char) in b.chars().rev().enumerate() {
        let a_char = a.chars().rev().nth(i).unwrap();
        let value = match (a_char, b_char, carry) {
            ('0', '0', false) => '0',
            ('0', '0', true) => {
                carry = false;
                '1'
            }
            ('1', '0', false) | ('0', '1', false) => '1',
            ('1', '0', true) | ('0', '1', true) => '0',
            ('1', '1', false) => {
                carry = true;
                '0'
            }
            ('1', '1', true) => '1',
            _ => '0',
        };
        sum.push(value);
    }
    if carry {
        sum.push('1');
    }
    sum.iter().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_success() {
        let a_1 = "11".to_string();
        let b_1 = "1".to_string();
        assert_eq!(add_binary(a_1, b_1), "100".to_string());

        let a_2 = "1010".to_string();
        let b_2 = "1011".to_string();
        assert_eq!(add_binary(a_2, b_2), "10101".to_string());

        let a_2 = "100".to_string();
        let b_2 = "110010".to_string();
        assert_eq!(add_binary(a_2, b_2), "110110".to_string());
    }
}
