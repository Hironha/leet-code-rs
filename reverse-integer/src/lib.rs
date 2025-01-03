// Really easy problem. Don't know why is medium level.
pub fn reverse(mut x: i32) -> i32 {
    let mut ans = 0i32;
    while x != 0 {
        ans = match ans.checked_mul(10).and_then(|v| v.checked_add(x % 10)) {
            Some(sum) => sum,
            None => return 0,
        };

        x /= 10;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = 123;
        assert_eq!(reverse(input), 321);
    }

    #[test]
    fn case2() {
        let input = -123;
        assert_eq!(reverse(input), -321);
    }

    #[test]
    fn case3() {
        let input = -120;
        assert_eq!(reverse(input), -21);
    }

    #[test]
    fn case4() {
        let input = 1534236469;
        assert_eq!(reverse(input), 0);
    }
}
