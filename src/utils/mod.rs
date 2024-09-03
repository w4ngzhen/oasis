use bevy::hierarchy::DespawnRecursiveExt;
use bevy::prelude::{Commands, Component, Entity, Query, With};

pub mod rand_gen;


pub fn destroy_entity<T: Component>(to_destroy: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_destroy {
        commands.entity(entity).despawn_recursive();
    }
}
