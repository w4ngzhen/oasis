use crate::in_main_menu::components::InMainMenuScreen;
use crate::in_main_menu::systems::{menu_button_system, setup_main_menu};
use crate::game_state::GameState;
use crate::utils::destroy_entity;
use bevy::prelude::*;

mod components;
mod systems;

pub struct InMainMenuPlugin;

impl Plugin for InMainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InMainMenu), setup_main_menu);
        app.add_systems(OnExit(GameState::InMainMenu), destroy_entity::<InMainMenuScreen>);
        app.add_systems(Update, menu_button_system.run_if(in_state(GameState::InMainMenu)));
    }
}
