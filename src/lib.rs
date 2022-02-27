#[cfg(feature = "img")]
pub mod ulamspiral_img;

pub mod lookup;

/// A quadrant that is useful in knowing where the x,y coordinate exist in a cartisan plan.
/// Any 2 directional value like (NorthWest) will be on a perfect diagonal (ex: x: -8, y: 8).
#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct UlamPoint {
    pub value: i32,
    pub coord: Coord,
    pub quad: Quad,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}

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
/// use ulam::{Coord, Quad};
/// let c1 = Coord::new(0, 1);
/// let result = ulam::get_ulam_deets(&c1);    
/// assert_eq!(result.quad, Quad::North);
/// ```
pub fn value_of_coord(q: &Quad, c: &Coord) -> i32 {
    match q {
        // n = y c = -x
        // 4n^2 - 1n + c
        Quad::North => 4 * (c.y * c.y) - c.y + (-c.x),
        // n = x c = y
        // 4n^2 - 3n + c
        Quad::East => 4 * (c.x * c.x) - (3 * c.x) + c.y,
        // n = -y c = x
        // 4n^2 + 3n + c
        Quad::South => 4 * (-c.y * -c.y) + (3 * -c.y) + c.x,
        // n = -x c = -y
        // 4n^2 + 1n + c
        Quad::West => 4 * (-c.x * -c.x) + (-c.x) + (-c.y),
        // 4n^2
        Quad::NorthWest => 4 * (c.x * c.x),
        // 4n*2 - 2n
        Quad::NorthEast => 4 * (c.x * c.x) - 2 * c.x,
        // 4n^2 + 2n // putting in abs to make this work
        Quad::SouthWest => 4 * (c.x * c.x) + 2 * c.x.abs(),
        // 4n^2 + 4n
        Quad::SouthEast => 4 * (c.x * c.x) + 4 * c.x,
        // middle
        Quad::Center => 0,
    }
}

pub fn get_ulam_deets(c: &Coord) -> UlamPoint {
    let q = quad_of_coord(&c);
    let x = value_of_coord(&q, &c);

    UlamPoint {
        value: x,
        coord: *c,
        quad: q,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_n() {
        let c1 = Coord::new(0, 1);
        let result = get_ulam_deets(&c1);

        assert_eq!(result.quad, Quad::North);
    }
    #[test]
    fn check_w() {
        let c1 = Coord::new(-1, 0);
        let result = get_ulam_deets(&c1);

        assert_eq!(result.quad, Quad::West);
    }
    #[test]
    fn check_s() {
        let c1 = Coord::new(0, -1);
        let result = get_ulam_deets(&c1);

        assert_eq!(result.quad, Quad::South);
    }
    #[test]
    fn check_e() {
        let c1 = Coord::new(1, 0);
        let result = get_ulam_deets(&c1);

        assert_eq!(result.quad, Quad::East);
    }
    #[test]
    fn check_ne() {
        let c1 = Coord::new(1, 1);
        let result = get_ulam_deets(&c1);

        assert_eq!(result.quad, Quad::NorthEast);
    }
    #[test]
    fn check_nw() {
        let c1 = Coord::new(-1, 1);
        let result = get_ulam_deets(&c1);

        assert_eq!(result.quad, Quad::NorthWest);
    }
    #[test]
    fn check_sw() {
        let c1 = Coord::new(-1, -1);
        let result = get_ulam_deets(&c1);

        assert_eq!(result.quad, Quad::SouthWest);
    }
    #[test]
    fn check_se() {
        let c1 = Coord::new(1, -1);
        let result = get_ulam_deets(&c1);

        assert_eq!(result.quad, Quad::SouthEast);
    }
    #[test]
    fn check_middle() {
        let c1 = Coord::new(0, 0);
        let result = get_ulam_deets(&c1);

        assert_eq!(result.quad, Quad::Center);
    }
    ///
    // value_of_coord tests
    #[test]
    fn check_n_val() {
        let c1 = Coord::new(-9, 10);
        let p = get_ulam_deets(&c1);
        let result = value_of_coord(&p.quad, &c1);
        assert_eq!(result, 399);
    }
    #[test]
    fn check_w_val() {
        let c1 = Coord::new(-6, -3);
        let p = get_ulam_deets(&c1);
        let result = value_of_coord(&p.quad, &c1);
        assert_eq!(result, 153);
    }
    #[test]
    fn check_e_val() {
        let c1 = Coord::new(8, -2);
        let p = get_ulam_deets(&c1);
        let result = value_of_coord(&p.quad, &c1);
        assert_eq!(result, 230);
    }
    #[test]
    fn check_s_val() {
        let c1 = Coord::new(0, -2);
        let p = get_ulam_deets(&c1);
        let result = value_of_coord(&p.quad, &c1);
        assert_eq!(result, 22);
    }
    #[test]
    fn check_se_val() {
        let c1 = Coord::new(9, -9);
        let p = get_ulam_deets(&c1);
        let result = value_of_coord(&p.quad, &c1);
        dbg!(result);
        assert_eq!(result, 360);
    }
    #[test]
    fn check_ne_val() {
        let c1 = Coord::new(2, 2);
        let p = get_ulam_deets(&c1);
        let result = value_of_coord(&p.quad, &c1);
        assert_eq!(result, 12);
    }
    #[test]
    fn check_nw_val() {
        let c1 = Coord::new(-3, 3);
        let p = get_ulam_deets(&c1);
        let result = value_of_coord(&p.quad, &c1);
        assert_eq!(result, 36);
    }
    #[test]
    fn check_sw_val() {
        let c1 = Coord::new(-9, -9);
        let p = get_ulam_deets(&c1);
        let result = value_of_coord(&p.quad, &c1);
        assert_eq!(result, 342);
    }
    #[test]
    fn check_e_val_big() {
        let c1 = Coord::new(400, -221);
        let p = get_ulam_deets(&c1);
        let result = value_of_coord(&p.quad, &c1);
        dbg!(result);
        assert_eq!(result, 638579);
    }
    #[test]
    fn check_w_val_big() {
        let c1 = Coord::new(-398, -129);
        let p = get_ulam_deets(&c1);
        let result = value_of_coord(&p.quad, &c1);
        assert_eq!(result, 634143);
    }
    #[test]
    fn check_s_val_big() {
        let c1 = Coord::new(-397, -996);
        let p = get_ulam_deets(&c1);
        let result = value_of_coord(&p.quad, &c1);
        assert_eq!(result, 3970655);
    }
    #[test]
    fn check_n_val_big() {
        let c1 = Coord::new(250, 999);
        let p = get_ulam_deets(&c1);
        let result = value_of_coord(&p.quad, &c1);
        assert_eq!(result, 3990755);
    }
}

// pub fn quad_of_coord(c: &Coord) -> Quad {
//     match (c.x == c.y, c.x == -c.y, c.x > 0, c.y > 0) {
//         (true, false, true, true) => Quad::NorthEast,
//         (true, false, false, false) => Quad::SouthWest,
//         (false, true, false, true) => Quad::NorthWest,
//         (false, true, true, false) => Quad::SouthEast,
//         _ => match (c.x > c.y, c.x > -c.y) {
//             (true, true) => Quad::East,
//             (false, false) => Quad::West,
//             (false, true) => Quad::North,
//             (true, false) => Quad::South
//         }
//     }
// }
