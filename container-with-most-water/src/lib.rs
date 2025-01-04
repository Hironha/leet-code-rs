// Also quite hard problem if you think by the wrong perspective. Could solve after reading the explanation
// from here https://leetcode.com/problems/container-with-most-water/solutions/5139915/video-simple-two-pointer-solution
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut l = 0usize;
    let mut r = height.len() - 1;
    while l < r {
        let h = height[l].min(height[r]);
        let w = r - l;
        let area = w as i32 * h;
        if area > max {
            max = area
        }

        if height[l] > height[r] {
            r -= 1;
        } else {
            l += 1;
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(max_area(input), 49);
    }

    #[test]
    fn case2() {
        let input = vec![1, 1];
        assert_eq!(max_area(input), 1);
    }

    #[test]
    fn case3() {
        let input = vec![8, 7, 2, 1];
        assert_eq!(max_area(input), 7);
    }

    #[test]
    fn case4() {
        let input = vec![1, 2, 1];
        assert_eq!(max_area(input), 2);
    }
}
