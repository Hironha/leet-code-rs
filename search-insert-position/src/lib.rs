pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let (mut low, mut high) = (0usize, nums.len());
    while low < high {
        let mid = low + (high - low) / 2;
        let val = nums[mid];

        use std::cmp::Ordering;
        match target.cmp(&val) {
            Ordering::Greater => low = mid + 1,
            Ordering::Less => high = mid,
            Ordering::Equal => return i32::try_from(mid).unwrap_or_default(),
        };
    }

    i32::try_from(low).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::search_insert;

    #[test]
    fn case1() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;

        assert_eq!(search_insert(nums, target), 2);
    }

    #[test]
    fn case2() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;

        assert_eq!(search_insert(nums, target), 1);
    }

    #[test]
    fn case3() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;

        assert_eq!(search_insert(nums, target), 4);
    }
}
