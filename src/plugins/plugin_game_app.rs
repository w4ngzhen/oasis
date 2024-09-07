use crate::components::MapPickCursor;
use crate::game_state::{GameState, PlayerTurnSubState};
use crate::resources::{MapCameraCenter, TileSize};
use crate::systems::fov::fov;
use crate::systems::game_camera::{spawn_game_camera, update_game_camera};
use crate::systems::game_hud::spawn_game_hud;
use crate::systems::game_map::{
    render_map_tile, setup_game_map, spawn_map_pick_cursor, spawn_map_tiles,
};
use crate::systems::monster_spawn::spawn_monster;
use crate::systems::movement::movement;
use crate::systems::player_input::{
    player_action_input, player_explore_input, player_picking_input, scale_map, setup_map_exploring,
};
use crate::systems::player_spawn::spawn_player;
use crate::utils::destroy_entity;
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
            OnTransition { exited: GameState::PrepareGame, entered: GameState::InGaming },
            (spawn_player, spawn_monster),
        );
        app.add_systems(
            Update,
            (player_action_input, movement)
                .chain()
                .run_if(in_state(PlayerTurnSubState::PlayerAction)),
        );
        app.add_systems(OnEnter(PlayerTurnSubState::MapExploring), setup_map_exploring)
            .add_systems(
                Update,
                player_explore_input.run_if(in_state(PlayerTurnSubState::MapExploring)),
            );
        app.add_systems(
            OnTransition {
                exited: PlayerTurnSubState::PlayerAction,
                entered: PlayerTurnSubState::MapPicking,
            },
            spawn_map_pick_cursor,
        )
        .add_systems(
            OnTransition {
                exited: PlayerTurnSubState::MapPicking,
                entered: PlayerTurnSubState::PlayerAction,
            },
            destroy_entity::<MapPickCursor>,
        )
        .add_systems(Update, player_picking_input.run_if(in_state(PlayerTurnSubState::MapPicking)));
        app.add_systems(
            Update,
            (update_game_camera, render_map_tile, scale_map, fov)
                .run_if(in_state(GameState::InGaming)),
        );
    }
}

fn finish_prepare_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::InGaming);
}
