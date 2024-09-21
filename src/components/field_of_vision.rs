use crate::components::position_2d::Position2d;
use bevy::prelude::*;

#[derive(Component)]
pub struct FieldOfVision {
    pub visible_tiles: Vec<Position2d>,
    pub range: i32,
    /// flag indicate the field of view changed.
    /// when "false" , means you should update the visible_tiles
    pub is_dirty_data: bool,
}

impl Default for FieldOfVision {
    fn default() -> Self {
        Self { visible_tiles: Vec::new(), range: 8, is_dirty_data: true }
    }
}
