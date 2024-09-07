use crate::components::field_of_vision::FieldOfVision;
use crate::components::position::Position;
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
        commands.spawn((
            MapTileElement { color: Color::WHITE, is_background: false },
            Player,
            FieldOfVision { visible_tiles: Vec::new(), range: 8, is_dirty_data: true },
            Position { x: player_init_pos.x, y: player_init_pos.y, z: 2 },
            SpriteBundle {
                sprite: Sprite { custom_size: Some(Vec2::new(1.0, 1.0)), ..Default::default() },
                texture: atlas.texture.clone(),
                ..Default::default()
            },
            TextureAtlas { layout: atlas.atlas.clone(), index: '@' as usize },
        ));
    } else {
        warn!("Attempted to spawn a player without a room.")
    }
}
