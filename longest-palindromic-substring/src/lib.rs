// Naive approach. Really slow. I think the time complexity of this approach is O(n³), but could be O(n²).
// Not really hard to implement in Rust, but needs to be careful. Easy to be off by one since it handles many indexes.
pub fn longest_palindrome(s: String) -> String {
    let bytes = s.as_bytes();

    let mut longest = &bytes[0..1];
    for i in 0..bytes.len() {
        let ahead = &bytes[i + 1..];

        let mut j = ahead.len();
        while let Some(pos) = last_position(&ahead[..j], &bytes[i]) {
            let real_pos = pos + i + 1;
            let palindrome = &bytes[i..real_pos + 1];

            if palindrome.len() > longest.len() && is_palindrome(palindrome) {
                longest = palindrome;
                break;
            }

            j -= 1;
        }
    }

    // SAFETY: guaranteed to be valid due input constraints
    String::from_utf8(longest.to_vec()).unwrap()
}

fn last_position<T: PartialEq>(slice: &[T], value: &T) -> Option<usize> {
    if let Some(i) = slice.iter().rev().position(|v| v == value) {
        return Some(slice.len() - i - 1);
    }

    None
}

fn is_palindrome(s: &[u8]) -> bool {
    if s.len() < 2 {
        return true;
    }

    let mut i = 0usize;
    let mut j = s.len() - 1;
    while i <= j {
        if s[i] != s[j] {
            return false;
        }

        i += 1;
        j = j.saturating_sub(1);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = String::from("babad");
        assert_eq!(longest_palindrome(input), String::from("bab"));
    }

    #[test]
    fn case2() {
        let input = String::from("cbbd");
        assert_eq!(longest_palindrome(input), String::from("bb"));
    }

    #[test]
    fn case3() {
        let input = String::from("a");
        assert_eq!(longest_palindrome(input), String::from("a"));
    }

    #[test]
    fn case4() {
        let input = String::from("ac");
        assert_eq!(longest_palindrome(input), String::from("a"));
    }

    #[test]
    fn case5() {
        let input = String::from("ccc");
        assert_eq!(longest_palindrome(input), String::from("ccc"));
    }

    #[test]
    fn case6() {
        let input = String::from("aacabdkacaa");
        assert_eq!(longest_palindrome(input), String::from("aca"));
    }
}
