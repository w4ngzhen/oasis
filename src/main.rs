use crate::systems::keyboard_input::keyboard_input;
use crate::systems::setup::{setup_assets, setup_player, setup_world_map};
use bevy::prelude::*;

mod base;
mod components;
mod constants;
mod resources;
mod systems;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup_assets)
        .add_systems(Startup, (setup_world_map, setup_player).chain())
        .add_systems(Update, keyboard_input)
        .run();
}
