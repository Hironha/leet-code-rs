pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
    // get first string and set strs to the remaining list
    let (first, strs) = match strs.len() {
        1 => return strs.remove(0),
        _ => (strs.swap_remove(0), strs),
    };

    let mut prefix = String::new();

    for (i, byte) in first.as_bytes().iter().enumerate() {
        // compare char of first with other strings in the list
        for bytes in strs.iter().map(String::as_bytes) {
            if bytes.get(i) != Some(byte) {
                return prefix;
            }
        }

        prefix.push(char::from(*byte));
    }

    prefix
}

#[cfg(test)]
mod tests {
    use super::longest_common_prefix;

    #[test]
    fn case1() {
        let strs = vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
        ];

        assert_eq!(longest_common_prefix(strs), String::from("fl"));
    }

    #[test]
    fn case2() {
        let strs = vec![
            String::from("dog"),
            String::from("racecar"),
            String::from("car"),
        ];

        assert_eq!(longest_common_prefix(strs), String::new());
    }
}
