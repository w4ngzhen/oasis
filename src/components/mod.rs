use bevy::prelude::*;

pub mod attributes;
pub mod role;

pub mod field_of_vision;
pub mod name;
pub mod position;

/// Map tile tag
#[derive(Component)]
pub struct MapTile;
