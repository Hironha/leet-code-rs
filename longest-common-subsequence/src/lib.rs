pub enum Algorithm {
    Recursive,
    Dynamic,
}

pub fn lcs(left: &str, right: &str, algorithm: Algorithm) -> usize {
    let left: Vec<char> = left.chars().collect();
    let right: Vec<char> = right.chars().collect();

    match algorithm {
        Algorithm::Recursive => recursive_lcs(&left, &right),
        Algorithm::Dynamic => dynamic_lcs(&left, &right),
    }
}

// Recursive approach
// Easiest to implement, but not really efficient
// Time complexity: O(2^m)
// Space complexity: O(1)
fn recursive_lcs(left: &[char], right: &[char]) -> usize {
    if left.is_empty() || right.is_empty() {
        return 0;
    }

    if left[0] == right[0] {
        return 1 + recursive_lcs(&left[1..], &right[1..]);
    }

    recursive_lcs(&left[1..], right).max(recursive_lcs(left, &right[1..]))
}

// Dynamic approach
// The ideia is simillar to the wagner fischer levenshtein algorithm, where
// we store the comparisons of each characters in a matrix of left x right
// chars. But in the dynamic approach, we allocate the matrix lazily, i.e.
// we keep in memory only the current row and the previous row of the matrix
// Time complexity: O(m * n)
// Space complexity: O(m)
fn dynamic_lcs(left: &[char], right: &[char]) -> usize {
    let size = right.len() + 1;
    let mut prev = vec![0; size];
    let mut curr = prev.clone();

    for i in 1..left.len() + 1 {
        for j in 1..size {
            if left[i - 1] == right[j - 1] {
                curr[j] = 1 + prev[j - 1];
            } else {
                curr[j] = prev[j].max(curr[j - 1]);
            }
        }

        prev.copy_from_slice(&curr);
    }

    curr[size - 1]
}

#[cfg(test)]
mod tests {
    use super::{lcs, Algorithm};

    #[test]
    fn case1() {
        let left = "abcde";
        let right = "ace";

        assert_eq!(lcs(left, right, Algorithm::Recursive), 3);
        assert_eq!(lcs(left, right, Algorithm::Dynamic), 3);
    }

    #[test]
    fn case2() {
        let left = "abc";
        let right = "abc";

        assert_eq!(lcs(left, right, Algorithm::Recursive), 3);
        assert_eq!(lcs(left, right, Algorithm::Dynamic), 3);
    }

    #[test]
    fn case3() {
        let left = "abc";
        let right = "def";

        assert_eq!(lcs(left, right, Algorithm::Recursive), 0);
        assert_eq!(lcs(left, right, Algorithm::Dynamic), 0);
    }

    #[test]
    fn case4() {
        let left = "aggtab";
        let right = "gxtxayb";

        assert_eq!(lcs(left, right, Algorithm::Recursive), 4);
        assert_eq!(lcs(left, right, Algorithm::Dynamic), 4);
    }
}
