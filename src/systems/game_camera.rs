use crate::components::camera::GameMapCamera;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

pub fn spawn_game_camera(mut commands: Commands) {
    // game map camera
    commands.spawn((
        GameMapCamera,
        RenderLayers::layer(0),
        Camera2d,
        Camera { order: 0, ..default() },
    ));
}
