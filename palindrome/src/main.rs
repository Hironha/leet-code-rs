mod palindrome {
    pub fn is_palindrome(num: i32) -> bool {
        let mut rev: i32 = 0;
        let mut num_copy = num;

        while num_copy > 0 {
            rev = rev * 10 + num_copy % 10;
            num_copy /= 10;
        }

        num == rev
    }
}

fn main() {
    let num: i32 = 1234554321;

    println!(
        "number: {} is palindrome? {}",
        num,
        palindrome::is_palindrome(num)
    );
}

#[cfg(test)]
mod palindrome_test {
    use super::palindrome;

    #[test]
    fn should_be_palindrome() {
        let num_1: i32 = 121;
        let num_2: i32 = 2332;
        let num_3: i32 = 123321;

        assert!(palindrome::is_palindrome(num_1));
        assert!(palindrome::is_palindrome(num_2));
        assert!(palindrome::is_palindrome(num_3));
    }

    #[test]
    fn should_not_be_palindrome() {
        let num_1: i32 = 123;
        let num_2: i32 = 122;
        let num_3: i32 = 1879345;

        assert!(!palindrome::is_palindrome(num_1));
        assert!(!palindrome::is_palindrome(num_2));
        assert!(!palindrome::is_palindrome(num_3));
    }
}
