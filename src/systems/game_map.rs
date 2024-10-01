use crate::components::bundles::element_render_bundle;
use crate::components::field_of_vision::FieldOfVision;
use crate::components::map_element::{
    MapElement, MapElementProp, SystemItemPickCursor,
};
use crate::components::msg::MapWantsToPick;
use crate::components::position_2d::{Position2d, PositionZIndex};
use crate::components::role::Player;
use crate::components::MapTileElement;
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
                        MapElementProp::get(map_ele),
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
        commands
            .spawn((
                MapElement::SystemItem,
                SystemItemPickCursor,
                MapTileElement { color: Color::srgba(1., 1., 1., 0.5) },
                Position2d::new(player_pos.x, player_pos.y),
                PositionZIndex(999),
                element_render_bundle(get_charset_index(14, 0), &charset_asset),
            ))
            .insert(Visibility::Visible);
        // at the same time, spawn an init WantsToPick msg.
        commands.spawn(MapWantsToPick { position: player_pos.clone() });
    } else {
        warn!("cannot get player position, so, we cannot spawn pick cursor");
    }
}

/// 渲染地图内容，其核心是将所有拥有MapElement组件的实体的置到对应位置
pub fn render_map_tile(
    mut q_map_ele: Query<(
        Entity,
        &MapElement,
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
        _,
        map_ele,
        tile_ele,
        ele_pos,
        ele_z_idx,
        mut ele_visibility,
        mut transform,
        mut sprite,
    ) in q_map_ele.iter_mut()
    {
        // 渲染地图上的元素，这些元素有些当前视野能看到的，有些是曾经看到过的地方
        let (visibility, is_visited_tile): (Visibility, bool) = if player_fov
            .visible_positions
            .iter()
            .any(|pos| *pos == *ele_pos)
        {
            // 当前玩家能看到的，则需要展示，且标记它为非见过的位置
            (Visibility::Visible, false)
        } else if mb.game_map.visited_positions.contains(ele_pos) {
            // 除了玩家当前正看到的，剩余“看过”的地方
            // 因为玩家正看大到的，肯定属于“看过”的，所以这里只会有其余“看过”的
            // “看过”的位置，我们仅会展示墙壁、地面，其余都将进入迷雾
            if *map_ele == MapElement::Floor || *map_ele == MapElement::Wall {
                (Visibility::Visible, true)
            } else {
                (Visibility::Hidden, true)
            }
        } else {
            (Visibility::Hidden, true)
        };
        *ele_visibility = visibility;
        // 不在视野中，但“看过”的，仅仅用一层灰色来展示
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
