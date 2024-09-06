use crate::components::field_of_vision::FieldOfVision;
use crate::components::position::Position;
use crate::components::role::Player;
use crate::components::{MapTileDrawable, MapTileElement};
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use crate::game_state::GameState;
use crate::resources::CharsetAsset;
use bevy::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    atlas: Res<CharsetAsset>,
    mut mb: ResMut<GameMapBuilder>,
) {
    let player_start = mb.player_init_pos.clone();

    commands.spawn((
        MapTileElement { color: Color::WHITE },
        Player,
        FieldOfVision { visible_tiles: Vec::new(), range: 8, is_dirty_data: true },
        Position { x: player_start.x, y: player_start.y, z: 2 },
        SpriteBundle {
            sprite: Sprite { custom_size: Some(Vec2::new(1.0, 1.0)), ..Default::default() },
            texture: atlas.texture.clone(),
            ..Default::default()
        },
        TextureAtlas { layout: atlas.atlas.clone(), index: '@' as usize },
    ));
}
