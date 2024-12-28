#![allow(dead_code)]

#[derive(Clone, Debug)]
struct Subchar {
    ch: u8,
    idx: usize,
}

// Naive solution, really easy to reason and implement. The problem is that it's quite slow, with worst case of O(n²).
fn naive_length_of_longest_substring(s: &str) -> i32 {
    let bytes = s.as_bytes();

    let mut max_length = 0i32;
    let mut subchars = Vec::<Subchar>::new();
    let mut i = 0usize;
    while i < bytes.len() {
        let ch = bytes[i];
        if let Some(subchar) = subchars.iter().find(|sc| sc.ch == ch) {
            i = subchar.idx;
            max_length = max_length.max(subchars.len() as i32);
            subchars.clear();
        } else {
            subchars.push(Subchar { ch, idx: i });
        }

        i += 1;
    }

    max_length.max(subchars.len() as i32)
}

// Similar idea to the naive, but without using extra allocation. The time complexity is still the same,
// but should be faster in practice since we avoid heap allocations.
fn sliding_window_length_of_longest_substring(s: &str) -> i32 {
    let bytes = s.as_bytes();

    let mut max = 0i32;
    let mut wstart = 0usize;
    let mut i = 0usize;
    while i < bytes.len() {
        let ch = bytes[i];
        if let Some(dup_pos) = bytes[wstart..i].iter().position(|b| *b == ch) {
            max = max.max((i - wstart) as i32);
            wstart += dup_pos + 1;
        }

        i += 1;
    }

    let length = (i - wstart) as i32;
    max.max(length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "abcabcbb";
        assert_eq!(naive_length_of_longest_substring(s), 3);
        assert_eq!(sliding_window_length_of_longest_substring(s), 3);
    }

    #[test]
    fn case2() {
        let s = "bbbbb";
        assert_eq!(naive_length_of_longest_substring(s), 1);
        assert_eq!(sliding_window_length_of_longest_substring(s), 1);
    }

    #[test]
    fn case3() {
        let s = "pwwkew";
        assert_eq!(naive_length_of_longest_substring(s), 3);
        assert_eq!(sliding_window_length_of_longest_substring(s), 3);
    }

    #[test]
    fn case4() {
        let s = " ";
        assert_eq!(naive_length_of_longest_substring(s), 1);
        assert_eq!(sliding_window_length_of_longest_substring(s), 1);
    }

    #[test]
    fn case5() {
        let s = "cdd";
        assert_eq!(naive_length_of_longest_substring(s), 2);
        assert_eq!(sliding_window_length_of_longest_substring(s), 2);
    }
}
