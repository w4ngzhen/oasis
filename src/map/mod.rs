use crate::game_state::GameState;
use crate::map::map_builder::system_build_map;
use crate::map::region_map::spawn_map_tiles;
use crate::systems::spawn_player::spawn_player;
use bevy::prelude::*;

pub mod map_builder;
pub mod region_map;
mod themes;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGaming),
            (system_build_map, spawn_map_tiles, spawn_player).chain(),
        );
    }
}
