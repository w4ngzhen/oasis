use crate::game_state::{GameState, PlayerTurnSubState};
use crate::resources::{MapCameraCenter, TileSize};
use crate::systems::game_camera::{spawn_game_camera, update_game_camera};
use crate::systems::game_hud::spawn_game_hud;
use crate::systems::game_map::{render_map_tile, setup_game_map, spawn_map_tiles};
use crate::systems::movement::movement;
use crate::systems::player_input::{
    player_explore_input, player_input, scale_map, setup_map_exploring,
};
use crate::systems::player_spawn::spawn_player;
use bevy::app::App;
use bevy::prelude::{
    in_state, IntoSystemConfigs, NextState, OnEnter, OnTransition, Plugin, ResMut, Update,
};

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
        app.add_systems(
            OnTransition { exited: GameState::PrepareGame, entered: GameState::PlayerTurn },
            spawn_player,
        );
        app.add_systems(
            Update,
            (player_input, movement).chain().run_if(in_state(PlayerTurnSubState::PlayerAction)),
        );
        app.add_systems(OnEnter(PlayerTurnSubState::MapExploring), setup_map_exploring)
            .add_systems(
                Update,
                player_explore_input.run_if(in_state(PlayerTurnSubState::MapExploring)),
            );
        app.add_systems(Update, (update_game_camera, render_map_tile, scale_map));
    }
}

fn finish_prepare_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::PlayerTurn);
}
