use crate::components::attributes::Attributes;
use crate::components::bundles::{element_render_bundle, player_bundle};
use crate::components::field_of_vision::FieldOfVision;
use crate::components::map_element::{MapElement, MapElementProp};
use crate::components::position_2d::{Position2d, PositionZIndex};
use crate::components::role::{Monster, Player};
use crate::components::{MapTileElement, Naming};
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
        commands.spawn((
            Player,
            Naming("player".into()),
            MapElement::Role,
            MapElementProp { is_collision: true, is_block_view: true },
            MapTileElement { color: Color::WHITE, ..default() },
            Attributes {
                max_hp: 100,
                current_hp: 100,
                damage: 20,
                defense: 10,
                ..default()
            },
            FieldOfVision { ..default() },
            Position2d { x: player_init_pos.x, y: player_init_pos.y },
            PositionZIndex(2),
            element_render_bundle('@' as usize, &charset_asset),
        ));
    } else {
        warn!("Attempted to spawn a player without a room.")
    }
}
