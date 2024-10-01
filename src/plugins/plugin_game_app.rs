use crate::components::map_element::SystemItemPickCursor;
use crate::game_state::{GameState, InGamingSubState};
use crate::resources::game_log::GameLog;
use crate::resources::{MapCameraCenter, TileSize};
use crate::systems::combat::{handle_combat, handle_object_destroy};
use crate::systems::end_turn::end_turn;
use crate::systems::fov::fov;
use crate::systems::game_camera::{spawn_game_camera, update_game_camera};
use crate::systems::game_hud::spawn_game_hud;
use crate::systems::game_map::{
    render_map_tile, setup_game_map, spawn_map, spawn_map_pick_cursor,
};
use crate::systems::monster_action::monster_chasing;
use crate::systems::monster_spawn::spawn_monster;
use crate::systems::movement::{monster_movement, player_movement};
use crate::systems::player_input::{
    pick_checking, player_explore_input, player_input, player_picking_input,
    scale_map, setup_map_exploring,
};
use crate::systems::player_spawn::spawn_player;
use crate::utils::destroy_entity;
use bevy::app::App;
use bevy::prelude::{
    in_state, IntoSystemConfigs, NextState, OnEnter, OnExit, OnTransition,
    Plugin, ResMut, Update,
};

pub struct GameAppPlugin;

impl Plugin for GameAppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapCameraCenter(None))
            .insert_resource(TileSize(16.))
            .insert_resource(GameLog::new());
        app.add_systems(
            OnTransition {
                exited: GameState::InMainMenu,
                entered: GameState::PrepareGame,
            },
            (
                spawn_game_camera,
                (setup_game_map, spawn_map).chain(),
                spawn_game_hud,
                finish_prepare_game,
            )
                .chain(),
        );
        app.add_systems(
            OnTransition {
                exited: GameState::PrepareGame,
                entered: GameState::InGaming,
            },
            (spawn_player, spawn_monster),
        );
        app.add_systems(
            Update,
            (update_game_camera, render_map_tile, scale_map, fov)
                .run_if(in_state(GameState::InGaming)),
        );
        app.add_systems(
            Update,
            player_input
                .run_if(in_state(InGamingSubState::AwaitingPlayerInput)),
        );
        app.add_systems(
            Update,
            (player_movement, handle_combat, handle_object_destroy, end_turn)
                .chain()
                .run_if(in_state(InGamingSubState::PlayerAction)),
        );
        app.add_systems(
            OnEnter(InGamingSubState::MapExploring),
            setup_map_exploring,
        )
        .add_systems(
            Update,
            player_explore_input
                .run_if(in_state(InGamingSubState::MapExploring)),
        );
        app.add_systems(
            OnEnter(InGamingSubState::MapPicking),
            spawn_map_pick_cursor,
        )
        .add_systems(
            Update,
            (player_picking_input, pick_checking)
                .chain()
                .run_if(in_state(InGamingSubState::MapPicking)),
        )
        .add_systems(
            OnExit(InGamingSubState::MapPicking),
            destroy_entity::<SystemItemPickCursor>,
        );
        app.add_systems(
            OnTransition {
                exited: InGamingSubState::PlayerAction,
                entered: InGamingSubState::MonsterAction,
            },
            (
                monster_chasing,
                monster_movement,
                handle_combat,
                handle_object_destroy,
                end_turn,
            )
                .chain(),
        );
    }
}

fn finish_prepare_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::InGaming);
}
