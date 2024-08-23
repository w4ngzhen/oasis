use bevy::prelude::Component;
use std::fmt::{Display, Formatter};

#[derive(Component, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Position {
    pub x: u64,
    pub y: u64,
}

impl Position {
    pub fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }
    pub fn from(data: [u64; 2]) -> Self {
        let [x, y] = data;
        Position::new(x, y)
    }
    pub fn to_tuple(&self) -> (u64, u64) {
        (self.x, self.y)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position[x={}, y={}]", self.x, self.y)
    }
}
