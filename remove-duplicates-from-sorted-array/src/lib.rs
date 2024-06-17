pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut prev: Option<i32> = None;
    nums.retain(move |x| {
        let keep = prev.map(|p| p.ne(x)).unwrap_or(true);
        prev = Some(*x);
        keep
    });

    i32::try_from(nums.len()).unwrap_or(0)
}

// Alternative solution that does not deduplicated the `Vec`, but still provides
// a correct answer to this specific problem due how the tests are done
pub fn remove_duplicates_hack(nums: &mut [i32]) -> i32 {
    if nums.len() <= 1 {
        return i32::try_from(nums.len()).unwrap_or(0);
    }

    // since `nums` is already sorted, we can skip the first value
    let mut k = 1usize;
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            // we "append" it and increment the length of the virtual `Vec`
            nums[k] = nums[i];
            k += 1;
        }
    }

    i32::try_from(k).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::{remove_duplicates, remove_duplicates_hack};

    fn validate(nums: &[i32], k: usize) -> bool {
        if k <= 1 {
            return true;
        }

        for i in 1..k {
            if nums[i - 1] > nums[i] {
                return false;
            }
        }

        true
    }

    mod normal {
        use super::{remove_duplicates, validate};
        #[test]
        fn case1() {
            let mut nums = vec![1, 1, 2];
            let k = remove_duplicates(&mut nums);

            assert!(validate(&nums, usize::try_from(k).unwrap_or(0)));
            assert_eq!(k, 2);
        }

        #[test]
        fn case2() {
            let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
            let k = remove_duplicates(&mut nums);

            assert!(validate(&nums, usize::try_from(k).unwrap_or(0)));
            assert_eq!(k, 5);
        }
    }

    mod hack {
        use super::{remove_duplicates_hack, validate};
        #[test]
        fn case1() {
            let mut nums = vec![1, 1, 2];
            let k = remove_duplicates_hack(&mut nums);

            println!("{k}: {nums:?}");

            assert!(validate(&nums, usize::try_from(k).unwrap_or(0)));
            assert_eq!(k, 2);
        }

        #[test]
        fn case2() {
            let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
            let k = remove_duplicates_hack(&mut nums);

            assert!(validate(&nums, usize::try_from(k).unwrap_or(0)));
            assert_eq!(k, 5);
        }
    }
}
