use crate::systems::setup::{setup, setup_assets};
use bevy::prelude::*;

mod components;
mod systems;
mod resources;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, (setup, setup_assets))
        .run();
}
