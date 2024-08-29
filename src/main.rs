use crate::game_state::GameState;
use crate::in_main_menu::InMainMenuPlugin;
use crate::in_player_config::InPlayerConfigPlugin;
use crate::in_portal::InPortalPlugin;
use bevy::prelude::*;

mod base;
mod components;
mod constants;
mod game_state;
mod in_gaming;
mod in_main_menu;
mod in_player_config;
mod in_portal;
mod map;
mod resources;
mod systems;
mod utils;

mod prelude {
    pub const REGION_TILE_WIDTH: u64 = 80;
    pub const REGION_TILE_HEIGHT: u64 = 80;
    pub const TILE_SIZE: u64 = 10;
    pub fn map_idx(x: u64, y: u64) -> usize {
        ((y * REGION_TILE_WIDTH) + x) as usize
    }
}

use crate::map::MapPlugin;
use crate::systems::map_render::{map_tile_position_translation, map_tile_size_scaling};
use crate::systems::setup::setup_charset_assets;
use prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
            primary_window: Some(Window {
                title: "Roguelike Game".to_string(),
                resolution:
                    (REGION_TILE_WIDTH as f32 * 10.0, REGION_TILE_HEIGHT as f32 * 10.0).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup_charset_assets)
        .add_systems(PostUpdate, (map_tile_size_scaling, map_tile_position_translation))
        .add_plugins(MapPlugin)
        .add_plugins((InPortalPlugin, InMainMenuPlugin, InPlayerConfigPlugin))
        // init_state一定要放在add_plugins之后
        .init_state::<GameState>()
        .run();
}
