pub fn length_of_last_word(s: String) -> i32 {
    let mut length = 0i32;

    for b in s.as_bytes().iter().rev() {
        match (b.is_ascii_whitespace(), length) {
            (false, _) => length += 1,
            (true, l) if l > 0 => return l,
            _ => (),
        };
    }

    length
}

#[cfg(test)]
mod tests {
    use super::length_of_last_word;

    #[test]
    fn case1() {
        let s = String::from("Hello World");

        assert_eq!(length_of_last_word(s), 5);
    }

    #[test]
    fn case2() {
        let s = String::from("   fly me   to   the moon  ");

        assert_eq!(length_of_last_word(s), 4);
    }

    #[test]
    fn case3() {
        let s = String::from("luffy is still joyboy");

        assert_eq!(length_of_last_word(s), 6);
    }
}
