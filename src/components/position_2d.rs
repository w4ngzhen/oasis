use bevy::prelude::*;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::ops;

#[derive(Component)]
pub struct PositionZIndex(pub(crate) i32);

#[derive(Component, Copy, Clone, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
pub struct Position2d {
    pub x: i32,
    pub y: i32,
}

impl Position2d {
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl ops::Add<Position2d> for Position2d {
    type Output = Position2d;
    fn add(mut self, rhs: Position2d) -> Position2d {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl ops::Sub<Position2d> for Position2d {
    type Output = Position2d;
    fn sub(mut self, rhs: Position2d) -> Position2d {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

impl Position2d {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn new_with_2d(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from(data: [i32; 2]) -> Self {
        let [x, y] = data;
        Position2d::new(x, y)
    }

    pub fn to_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl Display for Position2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position[x={}, y={}]", self.x, self.y)
    }
}
