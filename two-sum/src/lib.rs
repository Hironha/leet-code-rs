use std::collections::HashMap;

// Time complexity: O(n)
// Space complexity: O(n)
pub fn linear(numbers: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut pairs: HashMap<i32, usize> = HashMap::new();

    for (i, n) in numbers.iter().enumerate() {
        match pairs.remove(n) {
            // return (idx, i), because if idx is found, then it has been already computed
            // therefore idx < i
            Some(idx) => return Some((idx, i)),
            None => pairs.insert(target - n, i),
        };
    }

    None
}

// Time complexity: O(n²)
// Space complexity: O(1)
pub fn quadratic(numbers: &[i32], target: i32) -> Option<(usize, usize)> {
    for (i, n) in numbers.iter().enumerate() {
        for (j, m) in numbers.iter().enumerate() {
            if n + m == target && i != j {
                return Some((i, j));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{linear, quadratic};

    #[test]
    fn case1() {
        let numbers = [2, 7, 11, 15];
        let target = 9;

        assert_eq!(linear(&numbers, target), Some((0, 1)));
        assert_eq!(quadratic(&numbers, target), Some((0, 1)));
    }

    #[test]
    fn case2() {
        let numbers = [3, 2, 4];
        let target = 6;

        assert_eq!(linear(&numbers, target), Some((1, 2)));
        assert_eq!(quadratic(&numbers, target), Some((1, 2)));
    }

    #[test]
    fn case3() {
        let numbers = [3, 3];
        let target = 6;

        assert_eq!(linear(&numbers, target), Some((0, 1)));
        assert_eq!(quadratic(&numbers, target), Some((0, 1)));
    }
}
