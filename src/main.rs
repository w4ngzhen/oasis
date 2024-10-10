use crate::game_state::{GameState, InGamingSubState};
use crate::in_main_menu::InMainMenuPlugin;
use crate::in_player_config::InPlayerConfigPlugin;
use crate::in_portal::InPortalPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod components;
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

use crate::plugins::plugin_game_app::GameAppPlugin;
use crate::systems::setup::setup_charset_assets;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Oasis".to_string(),
                    resolution: (800., 600.).into(),
                    ..default()
                }),
                ..default()
            },
        ))
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup_charset_assets)
        .add_plugins(GameAppPlugin)
        .add_plugins((InPortalPlugin, InMainMenuPlugin, InPlayerConfigPlugin))
        .run();
}
