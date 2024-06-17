// using built-in `Vec::retain` method
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|num| *num != val);
    nums.len() as i32
}

pub fn remove_element_raw(nums: &mut [i32], val: i32) -> i32 {
    let mut k = 0usize;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[k] = nums[i];
            k += 1;
        }
    }

    i32::try_from(k).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    fn validate(nums: &[i32], expected: &[i32]) -> bool {
        if nums.len() != expected.len() {
            return false;
        }

        nums.iter()
            .zip(expected.iter())
            .all(|(left, right)| left == right)
    }

    mod builtin {
        use super::validate;
        use crate::remove_element;

        #[test]
        fn case1() {
            let mut nums = vec![3, 2, 2, 3];
            let k = remove_element(&mut nums, 3);
            let expected = &mut nums[0..usize::try_from(k).unwrap_or(0)];
            expected.sort();

            assert_eq!(k, 2);
            assert!(validate(expected, &[2, 2]))
        }

        #[test]
        fn case2() {
            let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
            let k = remove_element(&mut nums, 2);
            let expected = &mut nums[0..usize::try_from(k).unwrap_or(0)];
            expected.sort();

            assert_eq!(k, 5);
            assert!(validate(expected, &[0, 0, 1, 3, 4]));
        }
    }

    mod raw {
        use super::validate;
        use crate::remove_element_raw;

        #[test]
        fn case1() {
            let mut nums = vec![3, 2, 2, 3];
            let k = remove_element_raw(&mut nums, 3);
            let expected = &mut nums[0..usize::try_from(k).unwrap_or(0)];
            expected.sort();

            assert_eq!(k, 2);
            assert!(validate(expected, &[2, 2]))
        }

        #[test]
        fn case2() {
            let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
            let k = remove_element_raw(&mut nums, 2);
            let expected = &mut nums[0..usize::try_from(k).unwrap_or(0)];
            expected.sort();

            assert_eq!(k, 5);
            assert!(validate(expected, &[0, 0, 1, 3, 4]));
        }
    }
}
