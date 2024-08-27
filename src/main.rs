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
    pub const REGION_TILE_HEIGHT: u64 = 80;
    pub const REGION_TILE_WIDTH: u64 = 80;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((InPortalPlugin, InMainMenuPlugin, InPlayerConfigPlugin))
        // init_state一定要放在add_plugins之后
        .init_state::<GameState>()
        .run();
}
