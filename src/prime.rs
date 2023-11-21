use crate::{Coord, UlamPoint, quad_of_coord, value_of_coord};

pub fn is_prime(num: u32) -> bool {
    let num64 = num as u64;

    primal::is_prime(num64)
}

pub fn get_ulam_point(c: &Coord) -> UlamPoint {
    let q = quad_of_coord(c);
    let x = value_of_coord(c);

    UlamPoint {
        value: x,
        quad: q,
        is_prime: primal::is_prime(x.into()),
    }
}

#[cfg(test)]
mod tests {
    use crate::Quad;
    use super::*;
    #[test]
    fn check_n() {
        let c1 = Coord::new(0, 1);
        let result = get_ulam_point(&c1);

        assert_eq!(result.quad, Quad::North);
    }
    #[test]
    fn check_w() {
        let c1 = Coord::new(-1, 0);
        let result = get_ulam_point(&c1);

        assert_eq!(result.quad, Quad::West);
    }
    #[test]
    fn check_s() {
        let c1 = Coord::new(0, -1);
        let result = get_ulam_point(&c1);

        assert_eq!(result.quad, Quad::South);
    }
    #[test]
    fn check_e() {
        let c1 = Coord::new(1, 0);
        let result = get_ulam_point(&c1);

        assert_eq!(result.quad, Quad::East);
    }
    #[test]
    fn check_ne() {
        let c1 = Coord::new(1, 1);
        let result = get_ulam_point(&c1);

        assert_eq!(result.quad, Quad::NorthEast);
    }
    #[test]
    fn check_nw() {
        let c1 = Coord::new(-1, 1);
        let result = get_ulam_point(&c1);

        assert_eq!(result.quad, Quad::NorthWest);
    }
    #[test]
    fn check_sw() {
        let c1 = Coord::new(-1, -1);
        let result = get_ulam_point(&c1);

        assert_eq!(result.quad, Quad::SouthWest);
    }
    #[test]
    fn check_se() {
        let c1 = Coord::new(1, -1);
        let result = get_ulam_point(&c1);

        assert_eq!(result.quad, Quad::SouthEast);
    }
    #[test]
    fn check_middle() {
        let c1 = Coord::new(0, 0);
        let result = get_ulam_point(&c1);

        assert_eq!(result.quad, Quad::Center);
    }
}