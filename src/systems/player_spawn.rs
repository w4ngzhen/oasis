use crate::components::field_of_vision::FieldOfVision;
use crate::components::position::Position;
use crate::components::role::Player;
use crate::components::TileElement;
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use crate::core_module::{GAME_MAP_TILE_HEIGHT, GAME_MAP_TILE_WIDTH};
use crate::game_state::GameState;
use crate::resources::CharsetAsset;
use crate::utils::fov_utils::calc_fov;
use bevy::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    atlas: Res<CharsetAsset>,
    mut mb: ResMut<GameMapBuilder>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let player_start = mb.player_init_pos.clone();
    // calc fov
    let fov_range = 8;
    let visible_tiles =
        calc_fov(&player_start, fov_range, (GAME_MAP_TILE_WIDTH, GAME_MAP_TILE_HEIGHT), |pos| true);

    commands.spawn((
        TileElement,
        Player,
        FieldOfVision { visible_tiles, range: fov_range, invalid: false },
        Position { x: player_start.x, y: player_start.y, z: 2 },
        SpriteBundle {
            sprite: Sprite { custom_size: Some(Vec2::new(1.0, 1.0)), ..Default::default() },
            texture: atlas.texture.clone(),
            ..Default::default()
        },
        TextureAtlas { layout: atlas.atlas.clone(), index: '@' as usize },
    ));
}
