// input.iter -> transforms slice of strings into iterator
// .map() -> transforms slice of &String into slice of String
// .reduce() -> transform slice of String into a String
// .acc.chars() -> get a Iterator over acc characters
// .zip(cur.chars()) -> merge acc.chars() with cur.char() to create a Iterator over a tuple of the two
// .take_while() -> transforms the Iterator of tuples made above into a filtered Iterator
// .map() -> transform the filtered Iterator of tuples into a Iterator of chars
// .collect() -> transform the Iterator of chars into a String

pub fn functional_longest_common_prefix(input: &[String]) -> String {
    input
        .iter()
        .map(|str| str.to_string())
        .reduce(|acc, cur| {
            acc.chars()
                .zip(cur.chars())
                .take_while(|(a, c)| a == c)
                .map(|(a, _)| a)
                .collect()
        })
        .unwrap()
}

// slower than functional approach, probably because of format!() logic
pub fn procedural_longest_common_prefix(input: &[String]) -> String {
    if input.is_empty() {
        return String::new();
    }

    let test_word = input.first().unwrap();
    let mut common_prefix = String::from("");

    for char in test_word.chars() {
        let test_prefix = format!("{}{}", common_prefix, char);
        
        // checks if all strings starts with same prefix
        if input.iter().all(|str| str.starts_with(&test_prefix)) {
            common_prefix.push(char);
        } else {
            return common_prefix;
        }
    }

    common_prefix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_fl() {
        let input: Vec<String> = vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
        ];
        assert_eq!(functional_longest_common_prefix(&input), String::from("fl"));
        assert_eq!(procedural_longest_common_prefix(&input), String::from("fl"));
    }

    #[test]
    fn expect_empty() {
        let input: Vec<String> = vec![
            String::from("dog"),
            String::from("racecar"),
            String::from("car"),
        ];
        assert_eq!(functional_longest_common_prefix(&input), String::from(""));
        assert_eq!(procedural_longest_common_prefix(&input), String::from(""));
    }
}
