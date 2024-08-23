use crate::components::name::Name;
use crate::components::player::Player;
use crate::components::position::Position;
use bevy::log::info;
use bevy::prelude::*;

pub fn keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(&Name, &Position), With<Player>>,
) {
    info!("run keyboard input.");

    if keyboard_input.pressed(KeyCode::KeyA) {
        let (name, pos) = query.single();
        info!("'A' currently pressed, name = {}, pos = {}", name.0, pos);
    }

    if keyboard_input.just_pressed(KeyCode::KeyA) {
        info!("'A' just pressed");
    }
    if keyboard_input.just_released(KeyCode::KeyA) {
        info!("'A' just released");
    }
}
