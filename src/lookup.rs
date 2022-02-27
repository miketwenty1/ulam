/// Used to get an x,y coordinate given an integer.
use crate::{get_ulam_deets, Coord};

pub fn lookup(value: i32) -> Coord {
    if value < 0 {
        let c = Coord::new(0, 0);
        return c;
    }
    let x;
    let y;

    // divide value by 4, get the sqrt, then round to closest integer.
    let interim_val = ((value as f64) / 4.0).sqrt().round() as i32; // doesn't work for south quad numbers

    let nw_root = get_ulam_deets(&Coord::new(-interim_val, interim_val));
    let se_root;
    let ifdiff = ((value as f64) / 4.0).sqrt() - ((value as f64) / 4.0).sqrt().floor();

    // this is used to fix lookups on the south region.
    // this will change the se_root to the right root value.
    if ifdiff >= 0.0 && ifdiff < 0.5 {
        se_root = get_ulam_deets(&Coord::new(interim_val + 1, -interim_val));
    } else {
        se_root = get_ulam_deets(&Coord::new(interim_val, -interim_val + 1));
    }

    let nw_diff = nw_root.value - value;
    let se_diff = se_root.value - value;
    // we will assume the value is closer and inline with the NW root
    if se_diff.abs() > nw_diff.abs() {
        // check if nw_diff is positive or negative
        if nw_diff >= 0 {
            x = nw_root.coord.x + nw_diff;
            y = nw_root.coord.y;
        } else {
            x = nw_root.coord.x;
            y = nw_root.coord.y + nw_diff;
        }
    // we will assume the value is closer and inline with the SE root
    } else {
        // check if se_diff is positive or negative
        if se_diff >= 0 {
            x = se_root.coord.x - se_diff;
            y = se_root.coord.y;
        } else {
            x = se_root.coord.x;
            y = se_root.coord.y - se_diff;
        }
    }
    Coord::new(x, y)
}

#[cfg(test)]
mod tests {
    use super::{lookup, Coord};

    #[test]
    fn check_bad() {
        let result = lookup(-1);
        let c = Coord::new(0, 0);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_0() {
        let result = lookup(0);
        let c = Coord::new(0, 0);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_1() {
        let result = lookup(1);
        let c = Coord::new(1, 0);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_2() {
        let result = lookup(2);
        let c = Coord::new(1, 1);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_3() {
        let result = lookup(3);
        let c = Coord::new(0, 1);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_4() {
        let result = lookup(4);
        let c = Coord::new(-1, 1);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_5() {
        let result = lookup(5);
        let c = Coord::new(-1, 0);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_6() {
        let result = lookup(6);
        let c = Coord::new(-1, -1);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_7() {
        let result = lookup(7);
        let c = Coord::new(0, -1);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_8() {
        let result = lookup(8);
        let c = Coord::new(1, -1);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_9() {
        let result = lookup(9);
        let c = Coord::new(2, -1);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_10() {
        let result = lookup(10);
        let c = Coord::new(2, 0);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_11() {
        let result = lookup(11);
        let c = Coord::new(2, 1);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_12() {
        let result = lookup(12);
        let c = Coord::new(2, 2);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_13() {
        let result = lookup(13);
        let c = Coord::new(1, 2);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_14() {
        let result = lookup(14);
        let c = Coord::new(0, 2);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_15() {
        let result = lookup(15);
        let c = Coord::new(-1, 2);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_16() {
        let result = lookup(16);
        let c = Coord::new(-2, 2);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_17() {
        let result = lookup(17);
        let c = Coord::new(-2, 1);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_18() {
        let result = lookup(18);
        let c = Coord::new(-2, 0);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_19() {
        let result = lookup(19);
        let c = Coord::new(-2, -1);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_20() {
        let result = lookup(20);
        let c = Coord::new(-2, -2);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_21() {
        let result = lookup(21);
        let c = Coord::new(-1, -2);
        assert_eq!(result, c);
    }
    #[test]
    fn check_small_22() {
        let result = lookup(22);
        let c = Coord::new(0, -2);
        assert_eq!(result, c);
    }

    #[test]
    fn check_small_23() {
        let result = lookup(23);
        let c = Coord::new(1, -2);
        assert_eq!(result, c);
    }
    #[test]
    fn check_n1() {
        let result = lookup(12489);
        let c = Coord::new(-1, 56);
        assert_eq!(result, c);
    }
    #[test]
    fn check_n2() {
        let result = lookup(2022);
        let c = Coord::new(20, -22);
        assert_eq!(result, c);
    }
    #[test]
    fn check_n3() {
        let result = lookup(576);
        let c = Coord::new(-12, 12);
        assert_eq!(result, c);
    }
    #[test]
    fn check_n4() {
        let result = lookup(0);
        let c = Coord::new(0, 0);
        assert_eq!(result, c);
    }
    #[test]
    fn check_n5() {
        let result = lookup(1);
        let c = Coord::new(1, 0);
        assert_eq!(result, c);
    }
    #[test]
    fn check_deep_south() {
        let result = lookup(3987051);
        let c = Coord::new(41, -998);
        assert_eq!(result, c);
    }
}
