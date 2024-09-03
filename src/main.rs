use crate::game_state::GameState;
use crate::in_main_menu::InMainMenuPlugin;
use crate::in_player_config::InPlayerConfigPlugin;
use crate::in_portal::InPortalPlugin;
use bevy::prelude::*;

mod base;
mod components;
mod constants;
mod core_module;
mod game_state;
mod in_gaming;
mod in_main_menu;
mod in_player_config;
mod in_portal;
mod plugins;
mod resources;
mod systems;
mod utils;

use crate::core_module::*;
use crate::plugins::plugin_game_ui::GameUiPlugin;
use crate::plugins::plugin_layout::LayoutPlugin;
use crate::plugins::plugin_map::MapPlugin;
use crate::plugins::plugin_player::PlayerPlugin;
use crate::systems::render::{position_translation, tile_element_size_scaling};
use crate::systems::setup::setup_charset_assets;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
            primary_window: Some(Window {
                title: "Oasis".to_string(),
                resolution: (800., 600.).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup_charset_assets)
        .add_systems(PostUpdate, (tile_element_size_scaling, position_translation))
        .add_plugins(MapPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(GameUiPlugin)
        .add_plugins(LayoutPlugin)
        .add_plugins((InPortalPlugin, InMainMenuPlugin, InPlayerConfigPlugin))
        // init_state一定要放在add_plugins之后
        .init_state::<GameState>()
        .run();
}
