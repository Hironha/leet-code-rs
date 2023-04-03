pub fn remove_element(numbers: &mut Vec<i32>, val: i32) -> usize {
    numbers.retain(|num| *num != val);
    numbers.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_success() {
        let mut input_1: Vec<i32> = vec![1, 2, 3];
        assert_eq!(remove_element(&mut input_1, 3), 2);

        let mut input_2: Vec<i32> = vec![1, 2, 2, 3];
        assert_eq!(remove_element(&mut input_2, 2), 2);

        let mut input_3: Vec<i32> = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(remove_element(&mut input_3, 2), 5);
    }
}
