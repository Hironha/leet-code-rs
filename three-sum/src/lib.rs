use std::cmp::Ordering;

// Reference: https://leetcode.com/problems/3sum/solutions/5055810/video-two-pointer-solution
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();

    let mut ans = Vec::<Vec<i32>>::new();
    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut j = i + 1;
        let mut k = nums.len() - 1;
        while j < k {
            let sum = nums[i] + nums[j] + nums[k];
            match sum.cmp(&0) {
                Ordering::Less => {
                    j += 1;
                }
                Ordering::Greater => {
                    k -= 1;
                }
                Ordering::Equal => {
                    ans.push(vec![nums[i], nums[j], nums[k]]);
                    j += 1;

                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                }
            };
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_eq(left: &[i32], right: &[i32]) -> bool {
        for l in left.iter() {
            if !right.iter().any(|r| r == l) {
                return false;
            }
        }

        for l in right.iter() {
            if !left.iter().any(|r| r == l) {
                return false;
            }
        }

        true
    }

    fn triplet_arr_eq(left: &[Vec<i32>], right: &[Vec<i32>]) -> bool {
        if left.len() != right.len() {
            return false;
        }

        'outer: for l in right.iter() {
            for r in left.iter() {
                if set_eq(l, r) {
                    continue 'outer;
                }
            }

            return false;
        }

        true
    }

    #[test]
    fn case1() {
        let input = vec![-1, 0, 1, 2, -1, -4];
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        let out = three_sum(input);
        println!("{out:?}");
        assert!(triplet_arr_eq(&out, &expected));
    }

    #[test]
    fn case2() {
        let input = vec![0, 1, 1];
        let expected = Vec::<Vec<i32>>::new();
        assert!(triplet_arr_eq(&three_sum(input), &expected));
    }

    #[test]
    fn case3() {
        let input = vec![0, 0, 0];
        let expected = vec![vec![0, 0, 0]];
        assert!(triplet_arr_eq(&three_sum(input), &expected));
    }

    #[test]
    fn case4() {
        let input = vec![-4, -2, 1, -5, -4, -4, 4, -2, 0, 4, 0, -2, 3, 1, -5, 0];
        let expected = vec![
            vec![-5, 1, 4],
            vec![-4, 0, 4],
            vec![-4, 1, 3],
            vec![-2, -2, 4],
            vec![-2, 1, 1],
            vec![0, 0, 0],
        ];
        let out = three_sum(input);
        assert!(triplet_arr_eq(&out, &expected));
    }
}
