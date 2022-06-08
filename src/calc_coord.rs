/// Used to get an x,y coordinate given an integer on the ulam spiral.
use crate::Coord;

pub fn calc(value: u32) -> Coord {
    let x: i32;
    let y: i32;

    let n: i32 = (value as f64).sqrt().floor() as i32;
    let diff: i32 = value as i32 - (n * n);

    if n % 2 == 1 {
        // odd n arm
        if diff < n {
            x = (n + 1) / 2;
            y = ((1 - n) / 2) + diff;
        } else {
            x = (3 * n + 1) / 2 - diff;
            y = (n + 1) / 2;
        }
    } else {
        // even n arm
        if diff < n {
            x = -n / 2;
            y = n / 2 - diff;
        } else {
            x = ((-3 * n) / 2) + diff;
            y = -n / 2;
        }
    }
    return Coord { x: x, y: y };
}

#[cfg(test)]
mod tests {
    use super::{calc, Coord};

    #[test]
    fn check_small_0() {
        let result = calc(0);
        let c = Coord::new(0, 0);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_1() {
        let result = calc(1);
        let c = Coord::new(1, 0);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_2() {
        let result = calc(2);
        let c = Coord::new(1, 1);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_3() {
        let result = calc(3);
        let c = Coord::new(0, 1);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_4() {
        let result = calc(4);
        let c = Coord::new(-1, 1);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_5() {
        let result = calc(5);
        let c = Coord::new(-1, 0);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_6() {
        let result = calc(6);
        let c = Coord::new(-1, -1);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_7() {
        let result = calc(7);
        let c = Coord::new(0, -1);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_8() {
        let result = calc(8);
        let c = Coord::new(1, -1);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_9() {
        let result = calc(9);
        let c = Coord::new(2, -1);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_10() {
        let result = calc(10);
        let c = Coord::new(2, 0);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_11() {
        let result = calc(11);
        let c = Coord::new(2, 1);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_12() {
        let result = calc(12);
        let c = Coord::new(2, 2);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_13() {
        let result = calc(13);
        let c = Coord::new(1, 2);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_14() {
        let result = calc(14);
        let c = Coord::new(0, 2);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_15() {
        let result = calc(15);
        let c = Coord::new(-1, 2);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_16() {
        let result = calc(16);
        let c = Coord::new(-2, 2);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_17() {
        let result = calc(17);
        let c = Coord::new(-2, 1);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_18() {
        let result = calc(18);
        let c = Coord::new(-2, 0);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_19() {
        let result = calc(19);
        let c = Coord::new(-2, -1);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_20() {
        let result = calc(20);
        let c = Coord::new(-2, -2);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_21() {
        let result = calc(21);
        let c = Coord::new(-1, -2);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_22() {
        let result = calc(22);
        let c = Coord::new(0, -2);
        assert_eq!(result, c);
    }

    #[test]
    fn check_small_23() {
        let result = calc(23);
        let c = Coord::new(1, -2);
        assert_eq!(result, c);
    }
    #[test]
    fn check_n1() {
        let result = calc(12489);
        let c = Coord::new(-1, 56);
        assert_eq!(result, c);
    }
    #[test]
    fn check_n2() {
        let result = calc(2022);
        let c = Coord::new(20, -22);
        assert_eq!(result, c);
    }
    #[test]
    fn check_n3() {
        let result = calc(576);
        let c = Coord::new(-12, 12);
        assert_eq!(result, c);
    }
    #[test]
    fn check_n4() {
        let result = calc(0);
        let c = Coord::new(0, 0);
        assert_eq!(result, c);
    }
    #[test]
    fn check_n5() {
        let result = calc(1);
        let c = Coord::new(1, 0);
        assert_eq!(result, c);
    }
    #[test]
    fn check_deep_south() {
        let result = calc(3987051);
        let c = Coord::new(41, -998);
        assert_eq!(result, c);
    }
}
