// this is basically wagner fischer levenshtein algorithm, but using
// dynamic allocating the matrix
pub fn edit_distance(left: &str, right: &str) -> usize {
    if left.is_empty() || right.is_empty() {
        return left.len().max(right.len());
    }

    // convert to `Vec<char>` to support UTF-8
    let left: Vec<char> = left.chars().collect();
    let right: Vec<char> = right.chars().collect();
    let size = right.len() + 1;

    // first rows is always 0..=n, since it's comparing an empty string
    // to a string
    let mut prev: Vec<usize> = (0..size).collect();
    let mut curr = vec![0; size];

    for i in 1..left.len() + 1 {
        // first column is always 0..=n, since it's comparing an empty string
        // to a string
        curr[0] = i;

        for j in 1..size {
            if left[i - 1] == right[j - 1] {
                // get operations of previous substring
                curr[j] = prev[j - 1];
            } else {
                // get min operations required to substring equality
                curr[j] = 1 + prev[j - 1].min(prev[j]).min(curr[j - 1]);
            }
        }

        prev.copy_from_slice(&curr);
    }

    curr[size - 1]
}

#[cfg(test)]
mod tests {
    use super::edit_distance;

    #[test]
    fn case1() {
        assert_eq!(edit_distance("horse", "ros"), 3);
        assert_eq!(edit_distance("ros", "horse"), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(edit_distance("intention", "execution"), 5);
        assert_eq!(edit_distance("execution", "intention"), 5);
    }

    #[test]
    fn case3() {
        assert_eq!(edit_distance("", "a"), 1);
        assert_eq!(edit_distance("a", ""), 1);
    }
}
