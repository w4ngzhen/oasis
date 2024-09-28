use crate::components::bundles::element_render_bundle;
use crate::components::field_of_vision::FieldOfVision;
use crate::components::map_element::MapElementProperty;
use crate::components::msg::MapWantsToPick;
use crate::components::position_2d::{Position2d, PositionZIndex};
use crate::components::role::Player;
use crate::components::{MapPickCursor, MapTileElement};
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use crate::core_module::game_map::themes::{
    tile_to_render_descriptor, TileRenderDescriptor,
};
use crate::core_module::*;
use crate::game_state::GameState;
use crate::resources::{CharsetAsset, MapCameraCenter, TileSize};
use crate::utils::get_charset_index;
use bevy::prelude::*;

pub fn setup_game_map(mut commands: Commands) {
    // prepare camera
    let mb = GameMapBuilder::new();
    commands.insert_resource(mb);
}

pub fn spawn_map(
    mut commands: Commands,
    mb: Res<GameMapBuilder>,
    charset_asset: Res<CharsetAsset>,
) {
    for y in 0..GAME_MAP_TILE_HEIGHT {
        for x in 0..GAME_MAP_TILE_WIDTH {
            let idx = map_idx(x, y);
            let render_descriptor =
                tile_to_render_descriptor(mb.game_map.elements[idx]);

            if let Some(render_descriptor) = render_descriptor {
                let map_ele = mb.game_map.elements[idx];
                commands
                    .spawn((
                        map_ele,
                        MapElementProperty::get(map_ele),
                        MapTileElement {
                            color: render_descriptor.color,
                            ..default()
                        },
                        Position2d::new(x, y),
                        PositionZIndex(1),
                        element_render_bundle(
                            render_descriptor.tile_index,
                            &charset_asset,
                        ),
                    ))
                    .insert(Visibility::Hidden);
            }
        }
    }
}

pub fn spawn_map_pick_cursor(
    mut commands: Commands,
    query_player: Query<&Position2d, With<Player>>,
    charset_asset: Res<CharsetAsset>,
) {
    if let Ok(player_pos) = query_player.get_single() {
        commands.spawn((
            MapPickCursor,
            MapTileElement { color: Color::srgba(1., 1., 1., 0.5) },
            Position2d::new(player_pos.x, player_pos.y),
            PositionZIndex(999),
            Visibility::Visible,
            element_render_bundle(get_charset_index(14, 0), &charset_asset),
        ));
        // at the same time, spawn an init WantsToPick msg.
        commands.spawn(MapWantsToPick { position: player_pos.clone() });
    } else {
        warn!("cannot get player position, so, we cannot spawn pick cursor");
    }
}

/// 渲染地图内容，其核心是将相关TileElement放置到对应位置
pub fn render_map_tile(
    mut q: Query<(
        Entity,
        &MapTileElement,
        &Position2d,
        &PositionZIndex,
        &mut Visibility,
        &mut Transform,
        &mut Sprite,
    )>,
    query_player: Query<(&Position2d, &FieldOfVision), With<Player>>,
    map_camera_center: Res<MapCameraCenter>,
    mb: Res<GameMapBuilder>,
    tile_size: Res<TileSize>,
) {
    let (player_pos, player_fov) = query_player.single();
    let center_pos = if let Some(center) = map_camera_center.0 {
        center
    } else {
        *player_pos
    };
    let tile_size = tile_size.0;
    let base = Vec3::new(
        -(center_pos.x as f32) * tile_size,
        center_pos.y as f32 * tile_size,
        0.,
    );
    for (
        ele_entity,
        tile_ele,
        ele_pos,
        ele_z_idx,
        mut ele_visibility,
        mut transform,
        mut sprite,
    ) in q.iter_mut()
    {
        let (visibility, is_visited_tile): (Visibility, bool) =
            if player_fov.visible_tiles.iter().any(|pos| *pos == *ele_pos) {
                (Visibility::Visible, false)
            } else if mb.game_map.visited_tiles[map_idx(ele_pos.x, ele_pos.y)] {
                if mb
                    .game_map
                    .occupation
                    .values()
                    .any(|occupation| *occupation == ele_entity)
                {
                    // you can see, but it's an object on map(not wall or floor)
                    (Visibility::Hidden, true)
                } else {
                    (Visibility::Visible, true)
                }
            } else {
                (Visibility::Hidden, false)
            };
        *ele_visibility = visibility;
        if is_visited_tile {
            sprite.color = Color::srgba(1., 1., 1., 0.3);
        } else {
            sprite.color = tile_ele.color
        }
        transform.scale = Vec3::new(tile_size, tile_size, 1.);
        let tile_pixel_pos = Vec3::new(
            ele_pos.x as f32 * tile_size,
            -(ele_pos.y as f32) * tile_size,
            ele_z_idx.0 as f32,
        );
        transform.translation = tile_pixel_pos + base;
    }
}
