use crate::game_state::GameState;
use crate::systems::map_build::{build_map, spawn_map_tiles};
use bevy::prelude::*;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGaming),
            (build_map, spawn_map_tiles).chain(),
        );
    }
}
