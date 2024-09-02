use crate::components::position::Position;
use crate::components::TileElement;
use crate::core_module::*;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn convert_pos(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
    // 例如，屏幕宽度为1280，而游戏宽度为80，那么，tile_size就是水平方向上一个tile的宽度：16
    let tile_size = bound_window / bound_game;
    // 始终居中
    pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
}

/// 地图上每一个tile放置的到实际屏幕上的位置
pub fn position_translation(
    primary_query: Query<&Window>,
    mut q: Query<(&Position, &mut Transform)>,
) {
    if let Ok(primary) = primary_query.get_single() {
        for (pos, mut transform) in q.iter_mut() {
            transform.translation = Vec3::new(
                convert_pos(pos.x as f32, primary.width(), REGION_TILE_WIDTH as f32),
                convert_pos(
                    (pos.y + TILE_SIZE / 2) as f32,
                    primary.height(),
                    REGION_TILE_HEIGHT as f32,
                ),
                pos.z as f32,
            );
        }
    }
}

/// 界面上每个tile的宽高，会根据当前屏幕的大小动态计算
/// 例如，当前窗口的 width = 1280，地图tile宽度为 80，则这个tile的宽度大小应为 1280 / 80 = 16
/// 这里，我们通过查询带有 TileElement 组件的实体的Transform变换，对其进行修改
pub fn tile_element_size_scaling(
    primary_query: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&mut Transform), With<TileElement>>,
) {
    if let Ok(primary) = primary_query.get_single() {
        for (mut transform) in q.iter_mut() {
            let scale = Vec3::new(
                // 这里首先根据 屏幕尺寸 / tile地图尺寸，得到的是每个tile的实际尺寸
                // 在相关spawn代码中，我们将对应sprite的custom_size设置为了1x1，因此在这里直接进行缩放
                primary.width() / REGION_TILE_WIDTH as f32,
                primary.height() / REGION_TILE_HEIGHT as f32,
                1.0,
            );
            transform.scale = scale;
        }
    }
}
