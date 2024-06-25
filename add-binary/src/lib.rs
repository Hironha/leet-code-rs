pub fn add_binary(a: String, b: String) -> String {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let mut i = a.len();
    let mut j = b.len();
    let mut carry = false;
    let mut sum = Vec::with_capacity(i.max(j) + 1);

    while i > 0 || j > 0 || carry {
        let x = if i == 0 {
            b'0'
        } else {
            i -= 1;
            a_bytes[i]
        };

        let y = if j == 0 {
            b'0'
        } else {
            j -= 1;
            b_bytes[j]
        };

        match (x, y, carry) {
            (b'0', b'0', false) | (b'0', b'1', true) | (b'1', b'0', true) => sum.push('0'),
            (b'0', b'1', false) | (b'1', b'0', false) | (b'1', b'1', true) => sum.push('1'),
            (b'0', b'0', true) => {
                sum.push('1');
                carry = false;
            }
            (b'1', b'1', false) => {
                sum.push('0');
                carry = true;
            }
            _ => panic!("Received invalid binary string"),
        };
    }

    sum.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::add_binary;

    #[test]
    fn case1() {
        let a = String::from("11");
        let b = String::from("1");

        assert_eq!(add_binary(a, b), String::from("100"));
    }

    #[test]
    fn case2() {
        let a = String::from("1010");
        let b = String::from("1011");

        assert_eq!(add_binary(a, b), String::from("10101"));
    }
}
