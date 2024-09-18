use crate::components::attributes::Attributes;
use crate::components::bundles::player_bundle;
use crate::components::field_of_vision::FieldOfVision;
use crate::components::position_2d::{Position2d, PositionZIndex};
use crate::components::role::{Monster, Player};
use crate::components::MapTileElement;
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use crate::resources::CharsetAsset;
use bevy::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    charset_asset: Res<CharsetAsset>,
    mut mb: ResMut<GameMapBuilder>,
) {
    if let Some(first_room) = mb.rooms.first() {
        let player_init_pos = first_room.center();
        // spawn
        let player_entity =
            commands.spawn(player_bundle(player_init_pos, &charset_asset)).id();
        // record player
        mb.game_map.occupation.insert(player_init_pos, player_entity);
    } else {
        warn!("Attempted to spawn a player without a room.")
    }
}
