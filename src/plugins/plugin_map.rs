use crate::game_state::GameState;
use crate::systems::map::{setup_map, spawn_map_tiles};
use bevy::prelude::*;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGaming), (setup_map, spawn_map_tiles).chain());
    }
}
