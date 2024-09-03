use crate::components::position::Position;
use crate::components::TileElement;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// 地图上每一个tile放置的到实际屏幕上的位置
pub fn position_translation(
    primary_query: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Position, &mut Transform), With<TileElement>>,
) {
    if let Ok(primary) = primary_query.get_single() {
        for (pos, mut transform) in q.iter_mut() {
            transform.translation = Vec3::new(pos.x as f32 * 16., pos.y as f32 * 16., pos.z as f32);
        }
    }
}

pub fn tile_element_size_scaling(mut q: Query<&mut Transform, With<TileElement>>) {
    for mut transform in q.iter_mut() {
        let scale = Vec3::new(16., 16., 1.);
        transform.scale = scale;
    }
}
