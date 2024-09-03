use crate::components::{GameMapCamera, GameUiCamera};
use crate::core_module::ui::layout::calc_game_layout;
use bevy::math::{UVec2, Vec2};
use bevy::prelude::{info, Camera, Query, Window, With, Without};
use bevy::render::camera::Viewport;
use bevy::utils::default;
use bevy::window::PrimaryWindow;

pub fn camera_layout(
    query_window: Query<&Window, With<PrimaryWindow>>,
    mut query_game_map_camera: Query<&mut Camera, With<GameMapCamera>>,
    mut query_game_ui_camera: Query<&mut Camera, (With<GameUiCamera>, Without<GameMapCamera>)>,
) {
    if let Ok(primary_window) = query_window.get_single() {
        let win_size = primary_window.physical_size();
        let scale = primary_window.scale_factor();
        let layout = calc_game_layout(win_size, scale);
        info!("layout {:?}", layout);
        if let Ok(mut game_map_camera) = query_game_map_camera.get_single_mut() {
            game_map_camera.viewport = Some(Viewport {
                physical_size: vec2_to_uv2(layout.left.size()),
                physical_position: vec2_to_uv2(layout.left.min),
                ..default()
            });
        }
        if let Ok(mut game_ui_camera) = query_game_ui_camera.get_single_mut() {
            game_ui_camera.viewport = Some(Viewport {
                physical_size: vec2_to_uv2(layout.right.size()),
                physical_position: vec2_to_uv2(layout.right.min),
                ..default()
            });
        }
    }
}

fn vec2_to_uv2(vec: Vec2) -> UVec2 {
    UVec2::new(vec.x as u32, vec.y as u32)
}
