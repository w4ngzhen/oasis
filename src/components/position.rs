use bevy::prelude::Component;
use std::fmt::{Display, Formatter};
use std::ops;

#[derive(Component, Clone, Debug, Eq, Hash, Ord, PartialOrd)]
pub struct Position {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

impl ops::Add<Position> for Position {
    type Output = Position;
    fn add(mut self, rhs: Position) -> Position {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

impl PartialEq<Position> for Position {
    fn eq(&self, other: &Self) -> bool {
        // just check 2D position
        self.x == other.x && self.y == other.y
    }
}

impl Position {
    pub fn new(x: u64, y: u64, z: u64) -> Self {
        Self { x, y, z }
    }
    pub fn from(data: [u64; 3]) -> Self {
        let [x, y, z] = data;
        Position::new(x, y, z)
    }

    pub fn to_tuple(&self) -> (u64, u64, u64) {
        (self.x, self.y, self.z)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position[x={}, y={}]", self.x, self.y)
    }
}
