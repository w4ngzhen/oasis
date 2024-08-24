use crate::constants::TILE_WIDTH;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::prelude::{Commands, Component, Entity, Query, With};

pub mod rand_gen;

pub fn xy_idx(x: u64, y: u64) -> u64 {
    (y * TILE_WIDTH) + x
}

pub fn destroy_entity<T: Component>(to_destroy: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_destroy {
        commands.entity(entity).despawn_recursive();
    }
}
