// the solution is to calculate using a bottom up approach
// let's say you want to calculate the possibilities for n = 4
// if you're in the fourth step (n = 4), there's 0 possibilities
// if you're in the third step (n = 3), there's 1 possibility, that is to climb 1 step
// if you're in the second step (n = 2), there's 2 possibilities, that is to climb 1 step or 2 steps
// if you're in the first step (n = 1), the amount of possibilities is the sum of the possibilities in te second step + the third step
// if you're in the beginning (n = 0), the amount of possibilities is the sum of the possibilities of the first step + the second step

pub fn climb_stairs(n: i32) -> i32 {
    if n == 0  {
        return 0;
    }
    let mut one = 0;
    let mut two = 1;
    for _ in 0..(n+1) {
        let tmp = one;
        one += two;
        two = tmp;
    }
    one
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(climb_stairs(0), 0);
        assert_eq!(climb_stairs(1), 1);
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
        assert_eq!(climb_stairs(4), 5);
        assert_eq!(climb_stairs(5), 8);
    }
}
