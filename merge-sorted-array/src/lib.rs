pub fn merge_with_sort(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
    let (m, mut n) = (
        usize::try_from(m).unwrap_or(0),
        usize::try_from(n).unwrap_or(0),
    );

    while n > 0 {
        nums1[m + n - 1] = nums2[n - 1];
        n -= 1;
    }

    nums1.sort()
}

// solution found here: https://leetcode.com/problems/merge-sorted-array/solutions/621955/simple-and-concise-rust-solution-fill-from-the-end/?languageTags=rust
pub fn merge_without_sort(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
    let (mut m, mut n) = (
        usize::try_from(m).unwrap_or(0),
        usize::try_from(n).unwrap_or(0),
    );

    while n > 0 {
        if m > 0 && nums1[m - 1] > nums2[n - 1] {
            nums1[m + n - 1] = nums1[m - 1];
            m -= 1;
        } else {
            nums1[m + n - 1] = nums2[n - 1];
            n -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_success() {
        let mut nums1: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
        let mut nums2: Vec<i32> = vec![2, 5, 6];
        let len1 = 3;
        let len2 = i32::try_from(nums2.len()).unwrap_or(0);
        merge_without_sort(&mut nums1, len1, &mut nums2, len2);

        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }
}
