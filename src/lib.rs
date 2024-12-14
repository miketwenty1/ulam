#[cfg(feature = "prime")]
pub mod prime;
#[cfg(feature = "img")]
pub mod ulamspiral_img;

pub mod calc_coord;

use crate::calc_coord::calc_coord;
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
pub fn quad_of_coord(c: &Coord) -> Quad {
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
/// use ulam::get_value_from_xy;
/// let result = ulam::get_value_from_xy(3, 4);    
///
/// ```
pub fn get_value_from_xy(x: i32, y: i32) -> u32 {
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

/// Get the value from the ulam spiral given a Quad and a Coord.
/// # Examples
/// ```
/// use ulam::get_xy_from_value;
/// let result = ulam::get_xy_from_value(9);    
///
/// ```
pub fn get_xy_from_value(v: u32) -> (i32, i32) {
    let x: i32;
    let y: i32;

    let n: i32 = (v as f64).sqrt().floor() as i32;
    let diff: i32 = v as i32 - (n * n);

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
    (x, y)
}

pub fn quad_of_xy(x: i32, y: i32) -> Quad {
    let c = Coord { x, y };
    quad_of_coord(&c)
}

pub fn quad_of_value(v: u32) -> Quad {
    quad_of_coord(&calc_coord(v))
}

#[cfg(test)]
mod tests {
    use super::*;
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
