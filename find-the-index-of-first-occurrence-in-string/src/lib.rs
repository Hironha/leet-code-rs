pub fn str_str(haystack: &str, needle: &str) -> i32 {
    let haystack = haystack.as_bytes();
    let needle = needle.as_bytes();

    for i in 0..haystack.len() {
        let mut j = 0;
        let mut k = i;

        while k < haystack.len() && j < needle.len() && haystack[k] == needle[j] {
            j += 1;
            k += 1;
        }

        if j == needle.len() {
            return i32::try_from(i).unwrap_or(-1);
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::str_str;

    #[test]
    fn case1() {
        let haystack = "sadbutsad";
        let needle = "sad";

        assert_eq!(str_str(haystack, needle), 0);
    }

    #[test]
    fn case2() {
        let haystack = "leetcode";
        let needle = "leeto";

        assert_eq!(str_str(haystack, needle), -1);
    }

    #[test]
    fn case3() {
        let haystack = "aaa";
        let needle = "aaaa";

        assert_eq!(str_str(haystack, needle), -1);
    }
}
