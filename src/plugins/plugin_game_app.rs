use bevy::a11y::accesskit::Role::Portal;
use crate::game_state::GameState;
use crate::systems::game_camera::{spawn_game_camera, update_game_camera};
use crate::systems::game_hud::spawn_game_hud;
use crate::systems::game_map::{render_map_tile, setup_game_map, spawn_map_tiles};
use crate::systems::player_spawn::spawn_player;
use bevy::app::App;
use bevy::prelude::{IntoSystemConfigs, OnTransition, Plugin, Update};

pub struct GameAppPlugin;

impl Plugin for GameAppPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnTransition { exited: GameState::InMainMenu, entered: GameState::PrepareGame },
            (
                spawn_game_camera,
                (setup_game_map, spawn_map_tiles, spawn_player).chain(),
                spawn_game_hud,
            )
                .chain(),
        );
        app.add_systems(Update, (update_game_camera, render_map_tile));
    }
}
