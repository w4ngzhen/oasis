mod components;
mod systems;

use crate::game_state::GameState;
use crate::in_player_config::components::InPlayerConfigScreen;
use crate::in_player_config::systems::{
    play_config_keyboard_handle, player_config_systems, setup_player_config_screen,
};
use crate::utils::destroy_entity;
use bevy::prelude::*;

pub struct InPlayerConfigPlugin;

impl Plugin for InPlayerConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InPlayerConfig), setup_player_config_screen);
        app.add_systems(OnExit(GameState::InPlayerConfig), destroy_entity::<InPlayerConfigScreen>);
        app.add_systems(
            Update,
            (player_config_systems, play_config_keyboard_handle)
                .run_if(in_state(GameState::InPlayerConfig)),
        );
    }
}
