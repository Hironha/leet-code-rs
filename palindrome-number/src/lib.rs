pub fn is_palindrome(x: i32) -> bool {
    let mut rev = 0;
    let mut remaining = x;

    while remaining > 0 {
        // shift `rev` digits to the left and append `remaining` last digit
        // to `rev`
        rev = rev * 10 + remaining % 10;
        // remove `remaining` last digit
        remaining /= 10;
    }

    rev == x
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;

    #[test]
    fn case1() {
        assert!(is_palindrome(121));
    }

    #[test]
    fn case2() {
        assert!(!is_palindrome(-121));
    }

    #[test]
    fn case3() {
        assert!(!is_palindrome(10));
    }
}
