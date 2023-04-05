// high memory usage solution
pub fn get_last_word_length_1(input: &str) -> usize {
    match input.split_whitespace().last() {
        Some(w) => w.len(),
        None => 0,
    }
}

// low memory usage solution
pub fn get_last_word_length_2(input: &str) -> usize {
    input
        .trim_end()
        .chars()
        .rev()
        .take_while(|c| !c.is_whitespace())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_success_method_1() {
        let input_1 = "my name is hironha".to_string();
        assert_eq!(get_last_word_length_1(&input_1), 7);

        let input_2 = "madness".to_string();
        assert_eq!(get_last_word_length_1(&input_2), 7);

        let input_3 = "".to_string();
        assert_eq!(get_last_word_length_1(&input_3), 0);
    }

    #[test]
    fn expect_success_method_2() {
        let input_1 = "my name is hironha".to_string();
        assert_eq!(get_last_word_length_2(&input_1), 7);

        let input_2 = "madness".to_string();
        assert_eq!(get_last_word_length_2(&input_2), 7);

        let input_3 = "".to_string();
        assert_eq!(get_last_word_length_2(&input_3), 0);
    }
}
