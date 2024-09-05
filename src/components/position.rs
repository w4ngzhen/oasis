use bevy::prelude::Component;
use std::fmt::{Display, Formatter};
use std::ops;

#[derive(Component, Copy, Clone, Debug, Eq, Hash, Ord, PartialOrd)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Position {
    pub fn zero() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
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
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn new_with_2d(x: i32, y: i32) -> Self {
        Self { x, y, z: 0 }
    }

    pub fn from(data: [i32; 3]) -> Self {
        let [x, y, z] = data;
        Position::new(x, y, z)
    }

    pub fn to_tuple(&self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position[x={}, y={}]", self.x, self.y)
    }
}
