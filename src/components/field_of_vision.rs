use crate::components::position::Position;
use bevy::prelude::*;

#[derive(Component)]
pub struct FieldOfVision {
    pub visible_tiles: Vec<Position>,
    pub range: i32,
    /// flag indicate the field of view changed.
    pub invalid: bool,
}
