pub fn climb_stairs(n: i32) -> i32 {
    let mut two = 0;
    let mut one = 1;

    for _ in 0..n {
        (one, two) = (one + two, one);
    }

    one
}

#[cfg(test)]
mod tests {
    use super::climb_stairs;

    #[test]
    fn case1() {
        let n = 2;
        assert_eq!(climb_stairs(n), 2);
    }

    #[test]
    fn case2() {
        let n = 3;
        assert_eq!(climb_stairs(n), 3);
    }

    #[test]
    fn case3() {
        let n = 4;
        assert_eq!(climb_stairs(n), 5);
    }
}
