use crate::game_state::GameState;
use crate::systems::movement::movement;
use crate::systems::player_input::player_input;
use crate::systems::player_spawn::spawn_player;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::PlayerTurn), spawn_player);
        app.add_systems(
            Update,
            (player_input, movement).chain().run_if(in_state(GameState::PlayerTurn)),
        );
    }
}
