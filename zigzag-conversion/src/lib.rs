// This problem is actually quite hard to reason about. It's easy to fall into a way of thinking
// that will not result in a good solution.
pub fn convert(s: String, num_rows: i32) -> String {
    let num_rows = num_rows as usize;
    if num_rows == 1 {
        return s;
    }

    let mut rows = vec![String::new(); num_rows];
    let mut row_idx = 0;
    let mut going_up = false;

    for ch in s.chars() {
        rows[row_idx].push(ch);
        if row_idx == num_rows - 1 {
            going_up = false;
        } else if row_idx == 0 {
            going_up = true;
        }

        if going_up {
            row_idx += 1;
        } else {
            row_idx -= 1;
        }
    }

    rows.into_iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = String::from("PAYPALISHIRING");
        assert_eq!(convert(s, 3), String::from("PAHNAPLSIIGYIR"));
    }

    #[test]
    fn case2() {
        let s = String::from("PAYPALISHIRING");
        assert_eq!(convert(s, 4), String::from("PINALSIGYAHRPI"));
    }

    #[test]
    fn case3() {
        let s = String::from("A");
        assert_eq!(convert(s, 1), String::from("A"));
    }
}
