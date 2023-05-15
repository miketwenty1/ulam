#[cfg(feature = "img")]
pub mod ulamspiral_img;

pub mod calc_coord;
pub mod prime;

use serde::{Deserialize, Serialize};

/// A quadrant that is useful in knowing where the x,y coordinate exist in a cartisan plan.
/// Any 2 directional value like (NorthWest) will be on a perfect diagonal (ex: x: -8, y: 8).
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Quad {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    Center,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct UlamPoint {
    pub value: u32,
    pub quad: Quad,
    pub is_prime: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}

#[allow(clippy::comparison_chain)]
fn quad_of_coord(c: &Coord) -> Quad {
    if c.x == 0 && c.y == 0 {
        Quad::Center
    // Checking for Cartesian Q1
    } else if c.x >= 0 && c.y >= 0 {
        // check for north shard
        if c.y > c.x {
            Quad::North
        // check for east shard
        } else if c.x > c.y {
            Quad::East
        // check for diagonal
        } else {
            Quad::NorthEast
        }
    // Checking for Cartesian Q2
    } else if c.x <= 0 && c.y >= 0 {
        // check for north shard
        if c.y > c.x.abs() {
            Quad::North
        // check for west shard
        } else if c.x.abs() > c.y {
            Quad::West
        // check for diagonal
        } else {
            Quad::NorthWest
        }
    }
    // Checking for Cartesian Q3
    else if c.x <= 0 && c.y <= 0 {
        // check for south shard
        if c.y.abs() > c.x.abs() {
            Quad::South
        // check for west shard
        } else if c.x.abs() > c.y.abs() {
            Quad::West
        // check for diagonal
        } else {
            Quad::SouthWest
        }
    }
    // Checking for Cartesian Q4
    else if c.x >= 0 && c.y <= 0 {
        // check for south shard
        if c.y.abs() > c.x {
            Quad::South
        // check for west shard
        } else if c.x > c.y.abs() {
            Quad::East
        // check for diagonal
        } else {
            Quad::SouthEast
        }
    } else {
        Quad::Center
    }
}

/// Get the value from the ulam spiral given a Quad and a Coord.
/// # Examples
/// ```
/// use ulam::{Coord, value_of_coord};
/// let c1 = Coord::new(0, 1);
/// let result = ulam::value_of_coord(&c1);    
///
/// ```
pub fn value_of_coord(c: &Coord) -> u32 {
    let q = quad_of_coord(c);
    match q {
        // n = y c = -x
        // 4n^2 - 1n + c
        Quad::North => (4 * (c.y * c.y) - c.y + (-c.x)).try_into().unwrap(),
        // n = x c = y
        // 4n^2 - 3n + c
        Quad::East => (4 * (c.x * c.x) - (3 * c.x) + c.y).try_into().unwrap(),
        // n = -y c = x
        // 4n^2 + 3n + c
        Quad::South => (4 * (-c.y * -c.y) + (3 * -c.y) + c.x) as u32,
        // n = -x c = -y
        // 4n^2 + 1n + c
        Quad::West => (4 * (-c.x * -c.x) + (-c.x) + (-c.y)) as u32,
        // 4n^2
        Quad::NorthWest => (4 * (c.x * c.x)) as u32,
        // 4n*2 - 2n
        Quad::NorthEast => (4 * (c.x * c.x) - 2 * c.x) as u32,
        // 4n^2 + 2n // putting in abs to make this work
        Quad::SouthWest => (4 * (c.x * c.x) + 2 * c.x.abs()) as u32,
        // 4n^2 + 4n
        Quad::SouthEast => (4 * (c.x * c.x) + 4 * c.x) as u32,
        // middle
        Quad::Center => 0,
    }
}

/// Get the value from the ulam spiral given a Quad and a Coord.
/// # Examples
/// ```
/// use ulam::value_of_xy;
/// let result = ulam::value_of_xy(3, 4);    
///
/// ```
pub fn value_of_xy(x: i32, y: i32) -> u32 {
    let c = Coord { x, y };
    let q = quad_of_coord(&c);
    match q {
        // n = y c = -x
        // 4n^2 - 1n + c
        Quad::North => (4 * (c.y * c.y) - c.y + (-c.x)).try_into().unwrap(),
        // n = x c = y
        // 4n^2 - 3n + c
        Quad::East => (4 * (c.x * c.x) - (3 * c.x) + c.y).try_into().unwrap(),
        // n = -y c = x
        // 4n^2 + 3n + c
        Quad::South => (4 * (-c.y * -c.y) + (3 * -c.y) + c.x) as u32,
        // n = -x c = -y
        // 4n^2 + 1n + c
        Quad::West => (4 * (-c.x * -c.x) + (-c.x) + (-c.y)) as u32,
        // 4n^2
        Quad::NorthWest => (4 * (c.x * c.x)) as u32,
        // 4n*2 - 2n
        Quad::NorthEast => (4 * (c.x * c.x) - 2 * c.x) as u32,
        // 4n^2 + 2n // putting in abs to make this work
        Quad::SouthWest => (4 * (c.x * c.x) + 2 * c.x.abs()) as u32,
        // 4n^2 + 4n
        Quad::SouthEast => (4 * (c.x * c.x) + 4 * c.x) as u32,
        // middle
        Quad::Center => 0,
    }
}

pub fn get_ulam_point(c: &Coord) -> UlamPoint {
    let q = quad_of_coord(c);
    let x = value_of_coord(c);

    UlamPoint {
        value: x,
        quad: q,
        is_prime: prime::is_prime(x),
    }
}

#[cfg(test)]
mod tests {
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
    ///
    // value_of_coord tests
    #[test]
    fn check_n_val() {
        let c1 = Coord::new(-9, 10);

        let result = value_of_coord(&c1);
        assert_eq!(result, 399);
    }
    #[test]
    fn check_w_val() {
        let c1 = Coord::new(-6, -3);

        let result = value_of_coord(&c1);
        assert_eq!(result, 153);
    }
    #[test]
    fn check_e_val() {
        let c1 = Coord::new(8, -2);
        let result = value_of_coord(&c1);
        assert_eq!(result, 230);
    }
    #[test]
    fn check_s_val() {
        let c1 = Coord::new(0, -2);

        let result = value_of_coord(&c1);
        assert_eq!(result, 22);
    }
    #[test]
    fn check_se_val() {
        let c1 = Coord::new(9, -9);

        let result = value_of_coord(&c1);
        dbg!(result);
        assert_eq!(result, 360);
    }
    #[test]
    fn check_ne_val() {
        let c1 = Coord::new(2, 2);

        let result = value_of_coord(&c1);
        assert_eq!(result, 12);
    }
    #[test]
    fn check_nw_val() {
        let c1 = Coord::new(-3, 3);

        let result = value_of_coord(&c1);
        assert_eq!(result, 36);
    }
    #[test]
    fn check_sw_val() {
        let c1 = Coord::new(-9, -9);

        let result = value_of_coord(&c1);
        assert_eq!(result, 342);
    }
    #[test]
    fn check_e_val_big() {
        let c1 = Coord::new(400, -221);

        let result = value_of_coord(&c1);
        dbg!(result);
        assert_eq!(result, 638579);
    }
    #[test]
    fn check_w_val_big() {
        let c1 = Coord::new(-398, -129);

        let result = value_of_coord(&c1);
        assert_eq!(result, 634143);
    }
    #[test]
    fn check_s_val_big() {
        let c1 = Coord::new(-397, -996);

        let result = value_of_coord(&c1);
        assert_eq!(result, 3970655);
    }
    #[test]
    fn check_n_val_big() {
        let c1 = Coord::new(250, 999);

        let result = value_of_coord(&c1);
        assert_eq!(result, 3990755);
    }
}
