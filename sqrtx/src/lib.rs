pub fn my_sqrt(x: i32) -> i32 {
    let x = f64::from(x);
    let mut prev = 0f64;
    let mut root = x / 2.0;

    while (root - prev).abs() > 0.1 {
        prev = root;
        root = (root + x / root) / 2.0;
    }

    root.floor() as i32
}

#[cfg(test)]
mod tests {
    use super::my_sqrt;

    #[test]
    fn case1() {
        let x = 4;
        assert_eq!(my_sqrt(x), 2);
    }

    #[test]
    fn case2() {
        let x = 8;
        assert_eq!(my_sqrt(x), 2);
    }

    #[test]
    fn case3() {
        let x = 2147483647;
        assert_eq!(my_sqrt(x), 46340);
    }
}
