use crate::game_state::GameState;
use crate::resources::{MapCameraCenter, TileSize};
use crate::systems::game_camera::{spawn_game_camera, update_game_camera};
use crate::systems::game_hud::spawn_game_hud;
use crate::systems::game_map::{render_map_tile, setup_game_map, spawn_map_tiles};
use bevy::app::App;
use bevy::prelude::{IntoSystemConfigs, NextState, OnTransition, Plugin, ResMut, Update};

pub struct GameAppPlugin;

impl Plugin for GameAppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapCameraCenter(None)).insert_resource(TileSize(16.));
        app.add_systems(
            OnTransition { exited: GameState::InMainMenu, entered: GameState::PrepareGame },
            (
                spawn_game_camera,
                (setup_game_map, spawn_map_tiles).chain(),
                spawn_game_hud,
                finish_prepare_game,
            )
                .chain(),
        );
        app.add_systems(Update, (update_game_camera, render_map_tile));
    }
}

fn finish_prepare_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::PlayerTurn);
}
