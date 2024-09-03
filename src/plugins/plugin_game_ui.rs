use crate::game_state::GameState;
use crate::systems::ui::spawn_ui;
use bevy::app::{App, Plugin};
use bevy::prelude::OnEnter;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::PlayerTurn), spawn_ui);
    }
}
