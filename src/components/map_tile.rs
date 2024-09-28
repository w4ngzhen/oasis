use bevy::prelude::*;

#[derive(Component, Copy, Clone, PartialOrd, PartialEq)]
pub enum MapElementType {
    Void,
    Wall,
    Floor,
    Custom(i32),
}
