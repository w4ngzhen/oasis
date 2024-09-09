use crate::game_state::{GameState, InGamingSubState};
use crate::in_main_menu::InMainMenuPlugin;
use crate::in_player_config::InPlayerConfigPlugin;
use crate::in_portal::InPortalPlugin;
use bevy::prelude::*;

mod base;
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
use crate::plugins::plugin_player::PlayerPlugin;
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
        // .insert_resource(ClearColor(Color::Srgba(Srgba::new(1., 0.5, 0.5, 1.))))
        .add_systems(Startup, setup_charset_assets)
        .add_plugins(GameAppPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins((InPortalPlugin, InMainMenuPlugin, InPlayerConfigPlugin))
        // init_state一定要放在add_plugins之后
        .init_state::<GameState>()
        .add_sub_state::<InGamingSubState>()
        .run();
}
