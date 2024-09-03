use crate::components::position::Position;
use crate::components::{GameMapCamera, TileElement};
use crate::core_module::*;
use crate::game_state::GameState;
use crate::map::map_builder::MapBuilder;
use crate::map::themes::{tile_to_render_descriptor, TileRenderDescriptor};
use crate::resources::CharsetAsset;
use bevy::prelude::*;
use bevy::render::camera::Viewport;

pub fn setup_map(mut commands: Commands, query: Query<&Window>) {
    // prepare camera
    let window = query.single();
    let size = window.physical_size() / 2;
    commands.spawn((
        GameMapCamera,
        Camera2dBundle {
            camera: Camera {
                order: 1,
                viewport: Some(Viewport { physical_size: size, ..default() }),
                ..default()
            },
            ..default()
        },
    ));
    let mb = MapBuilder::new();
    commands.insert_resource(mb);
}

pub fn spawn_map_tiles(
    mut commands: Commands,
    mb: Res<MapBuilder>,
    charset_asset: Res<CharsetAsset>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for y in 0..REGION_TILE_HEIGHT {
        for x in 0..REGION_TILE_WIDTH {
            let idx = map_idx(x, y);
            let render_descriptor = tile_to_render_descriptor(mb.region_map.tiles[idx]);

            if let Some(render_descriptor) = render_descriptor {
                utils_spawn_map_tile_sprite(
                    &mut commands,
                    &charset_asset,
                    &render_descriptor,
                    &Position::new(x, y, 0),
                );
            }
        }
    }
    next_state.set(GameState::PlayerTurn);
}

fn utils_spawn_map_tile_sprite(
    commands: &mut Commands,
    charset_asset: &Res<CharsetAsset>,
    tile_render_descriptor: &TileRenderDescriptor,
    pos: &Position,
) {
    // 背景色
    if let Some(bg_color) = tile_render_descriptor.bg_color {
        commands.spawn((
            TileElement,
            Position { x: pos.x, y: pos.y, z: 0 }, // z = 0, background.
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
    commands.spawn((
        TileElement,
        Position { x: pos.x, y: pos.y, z: 1 }, // z = 1, foreground.
        SpriteBundle {
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
}
