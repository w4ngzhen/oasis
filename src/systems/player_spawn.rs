use crate::components::attributes::Attributes;
use crate::components::field_of_vision::FieldOfVision;
use crate::components::position_2d::{Position2d, PositionZIndex};
use crate::components::role::{Monster, Player};
use crate::components::MapTileElement;
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use crate::resources::CharsetAsset;
use bevy::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    atlas: Res<CharsetAsset>,
    mut mb: ResMut<GameMapBuilder>,
) {
    if let Some(first_room) = mb.rooms.first() {
        let player_init_pos = first_room.center();
        // spawn
        let player_entity = commands
            .spawn((
                MapTileElement { color: Color::WHITE, is_background: false },
                Player,
                Attributes {
                    strength: 0,
                    dexterity: 0,
                    toughness: 0,
                    max_hp: 100,
                    current_hp: 100,
                    max_energy: 0,
                    current_energy: 0,
                    damage: 20,
                    defense: 10,
                },
                FieldOfVision { visible_tiles: Vec::new(), range: 8, is_dirty_data: true },
                Position2d { x: player_init_pos.x, y: player_init_pos.y },
                PositionZIndex(2),
                SpriteBundle {
                    sprite: Sprite { custom_size: Some(Vec2::new(1.0, 1.0)), ..Default::default() },
                    texture: atlas.texture.clone(),
                    ..Default::default()
                },
                TextureAtlas { layout: atlas.atlas.clone(), index: '@' as usize },
            ))
            .id();
        // record player
        mb.game_map.occupation.insert(player_init_pos, player_entity);
    } else {
        warn!("Attempted to spawn a player without a room.")
    }
}
