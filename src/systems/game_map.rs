use crate::components::field_of_vision::FieldOfVision;
use crate::components::position_2d::{Position2d, PositionZIndex};
use crate::components::role::Player;
use crate::components::{MapPickCursor, MapTileElement};
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use crate::core_module::game_map::themes::{tile_to_render_descriptor, TileRenderDescriptor};
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

pub fn spawn_map_tiles(
    mut commands: Commands,
    mb: Res<GameMapBuilder>,
    charset_asset: Res<CharsetAsset>,
    mut _next_state: ResMut<NextState<GameState>>,
) {
    for y in 0..GAME_MAP_TILE_HEIGHT {
        for x in 0..GAME_MAP_TILE_WIDTH {
            let idx = map_idx(x, y);
            let render_descriptor = tile_to_render_descriptor(mb.game_map.tiles[idx]);

            if let Some(render_descriptor) = render_descriptor {
                utils_spawn_map_tile_sprite::<()>(
                    &mut commands,
                    &charset_asset,
                    &render_descriptor,
                    &Position2d::new(x, y),
                    1,
                    Visibility::Hidden, // default hidden
                    None,
                );
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
        utils_spawn_map_tile_sprite(
            &mut commands,
            &charset_asset,
            &TileRenderDescriptor::new(
                get_charset_index(14, 0, 16),
                Color::srgba(1., 1., 1., 0.5),
                None,
            ),
            &Position2d { x: player_pos.x, y: player_pos.y },
            999,
            Visibility::Visible,
            Some(MapPickCursor),
        );
    } else {
        warn!("cannot get player position, so, we cannot spawn pick cursor");
    }
}

fn utils_spawn_map_tile_sprite<TBundle>(
    commands: &mut Commands,
    charset_asset: &Res<CharsetAsset>,
    tile_render_descriptor: &TileRenderDescriptor,
    pos: &Position2d,
    z_index: i32,
    visibility: Visibility,
    extend_bundle: Option<TBundle>,
) where
    TBundle: Bundle,
{
    // 背景色
    if let Some(bg_color) = tile_render_descriptor.bg_color {
        commands.spawn((
            MapTileElement { color: bg_color, is_background: true },
            Position2d { x: pos.x, y: pos.y }, // z = 0, background.
            PositionZIndex(0),                 // background always.
            SpriteBundle {
                sprite: Sprite {
                    color: bg_color,
                    // 所有的图块初始都是 1x1的尺寸
                    // 我们会在相关系统中进行缩放处理
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ));
    }
    let mut cmds = commands.spawn((
        MapTileElement { color: tile_render_descriptor.color, is_background: false },
        Position2d { x: pos.x, y: pos.y },
        PositionZIndex(z_index),
        SpriteBundle {
            visibility,
            sprite: Sprite {
                color: tile_render_descriptor.color,
                // 所有的图块初始都是 1x1的尺寸
                // 我们会在相关系统中进行缩放处理
                custom_size: Some(Vec2::new(1., 1.)),
                ..Default::default()
            },
            texture: charset_asset.texture.clone(),
            ..Default::default()
        },
        TextureAtlas {
            layout: charset_asset.atlas.clone(),
            index: tile_render_descriptor.tile_index,
        },
    ));
    if let Some(bundle) = extend_bundle {
        cmds.insert(bundle);
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
    let center_pos = if let Some(center) = map_camera_center.0 { center } else { *player_pos };
    let tile_size = tile_size.0;
    let base = Vec3::new(-(center_pos.x as f32) * tile_size, center_pos.y as f32 * tile_size, 0.);
    for (ele_entity, tile_ele, ele_pos, ele_z_idx, mut ele_visibility, mut transform, mut sprite) in
        q.iter_mut()
    {
        let (visibility, is_visited_tile): (Visibility, bool) =
            if player_fov.visible_tiles.iter().any(|pos| *pos == *ele_pos) {
                (Visibility::Visible, false)
            } else if mb.game_map.visited_tiles[map_idx(ele_pos.x, ele_pos.y)] {
                if mb.game_map.occupation.values().any(|occupation| *occupation == ele_entity) {
                    // you can see, but it's an object on map(not wall or floor)
                    (Visibility::Hidden, true)
                } else {
                    if tile_ele.is_background {
                        (Visibility::Hidden, true)
                    } else {
                        (Visibility::Visible, true)
                    }
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
