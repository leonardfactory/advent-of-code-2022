use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

pub trait IntoPosValue {
    fn into_coordinate(self) -> i32;
}

impl IntoPosValue for i32 {
    fn into_coordinate(self) -> i32 {
        self
    }
}

impl IntoPosValue for usize {
    fn into_coordinate(self) -> i32 {
        self as i32
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Pos {
    pub const ZERO: Pos = Pos { x: 0, y: 0 };

    /// Directions
    pub const NW: Pos = Pos { x: -1, y: -1 };
    pub const N: Pos = Pos { x: 0, y: -1 };
    pub const NE: Pos = Pos { x: 1, y: -1 };
    pub const W: Pos = Pos { x: -1, y: 0 };
    pub const E: Pos = Pos { x: 1, y: 0 };
    pub const SW: Pos = Pos { x: -1, y: 1 };
    pub const S: Pos = Pos { x: 0, y: 1 };
    pub const SE: Pos = Pos { x: 1, y: 1 };

    pub fn new<T: IntoPosValue>(x: T, y: T) -> Self {
        Self {
            x: x.into_coordinate(),
            y: y.into_coordinate(),
        }
    }

    /// Returns only direct neighbors of this position (N, W, E, S) excluding
    /// diagonals
    pub fn neighbors(&self) -> Vec<Pos> {
        vec![
            // North
            *self + Self::N,
            // Center
            *self + Self::W,
            *self + Self::E,
            // Right
            *self + Self::S,
        ]
    }

    /// Returns all neighbors of this position, including diagonals (NE, NW, SE, SW)
    pub fn all_neighbors(&self) -> Vec<Pos> {
        vec![
            // North
            *self + Self::NW,
            *self + Self::N,
            *self + Self::NE,
            // Center
            *self + Self::W,
            *self + Self::E,
            // Right
            *self + Self::SW,
            *self + Self::S,
            *self + Self::SE,
        ]
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_additions() {
        assert_eq!(Pos::new(1, 1) + Pos::new(1, 1), Pos::new(2, 2));
        assert_eq!(Pos::new(1, 1) + Pos::new(-1, -1), Pos::new(0, 0));
    }
}
