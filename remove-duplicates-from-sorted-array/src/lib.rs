// solution using Vec built in methods 
pub fn remove_duplicates_built_in(arr: &mut Vec<i32>) -> usize {
    arr.dedup();
    arr.len()
}

// solution using custom logic
pub fn remove_duplicates_raw(arr: &mut Vec<i32>) -> usize {
    let mut previous: Option<i32> = None;

    arr.retain(|n| {
        let retain = match previous {
            Some(p) => p != *n,
            None => true,
        };
        previous = Some(*n);
        retain
    });
    arr.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_built_in_success() {
        let mut input1: Vec<i32> = vec![1, 1, 2];
        let expect1: Vec<i32> = vec![1, 2];

        assert_eq!(remove_duplicates_built_in(&mut input1), expect1.len());
        assert_eq!(&input1, &expect1);
    }

    #[test]
    fn expect_raw_success() {
        let mut input1: Vec<i32> = vec![1, 1, 2];
        let expect1: Vec<i32> = vec![1, 2];

        assert_eq!(remove_duplicates_raw(&mut input1), expect1.len());
        assert_eq!(&input1, &expect1);
    }
}
