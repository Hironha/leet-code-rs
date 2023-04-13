pub fn str_str(haystack: String, needle: String) -> i32 {
    // .find method is similar to indexOf in javascript
    haystack.find(&needle).map(|i| i as i32).unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_success() {
        let haystack_1 = "sadbutsad".to_string();
        let needle_1 = "sad".to_string();
        assert_eq!(str_str(haystack_1, needle_1), 0);

        let haystack_1 = "leetcode".to_string();
        let needle_1 = "leeto".to_string();
        assert_eq!(str_str(haystack_1, needle_1), -1);

        let haystack_1 = "hello".to_string();
        let needle_1 = "ll".to_string();
        assert_eq!(str_str(haystack_1, needle_1), 2);
    }
}
