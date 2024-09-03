use crate::components::{GameHudCamera, GameMapCamera};
use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::window::{PrimaryWindow, WindowResized};

pub fn spawn_game_camera(
    mut commands: Commands,
    query_window: Query<&Window, With<PrimaryWindow>>,
) {
    let primary_window = query_window.single();
    let layout = calc_game_layout(primary_window);
    // game map camera
    commands.spawn((
        GameMapCamera,
        Camera2dBundle {
            camera: Camera { order: 1, viewport: Some(layout.left_viewport), ..default() },
            ..default()
        },
    ));
    // game HUD camera
    commands.spawn((
        GameHudCamera,
        Camera2dBundle {
            camera: Camera { order: 999, viewport: Some(layout.right_viewport), ..default() },
            ..default()
        },
    ));
}

pub fn update_game_camera(
    query_window: Query<&Window, With<PrimaryWindow>>,
    mut resize_event: EventReader<WindowResized>,
    mut query_map_camera: Query<&mut Camera, With<GameMapCamera>>,
    mut query_hud_camera: Query<&mut Camera, (With<GameHudCamera>, Without<GameMapCamera>)>,
) {
    for resized in resize_event.read() {
        if let Ok(win) = query_window.get(resized.window) {
            let layout = calc_game_layout(win);
            if let Ok(mut map_camera) = query_map_camera.get_single_mut() {
                map_camera.viewport = Some(layout.left_viewport);
            }
            if let Ok(mut hud_camera) = query_hud_camera.get_single_mut() {
                hud_camera.viewport = Some(layout.right_viewport);
            }
        }
    }
}
#[derive(Debug)]
struct GameLayout {
    left_viewport: Viewport,
    right_viewport: Viewport,
}

/// 根据窗体尺寸，计算各个模块摄像机的viewport
pub fn calc_game_layout(window: &Window) -> GameLayout {
    let scale = window.scale_factor();
    let win_size = window.physical_size();
    let right_width = (200. * scale) as u32;
    let left_viewport = Viewport {
        physical_size: UVec2::new(win_size.x - right_width, win_size.y),
        physical_position: UVec2::new(0, 0),
        ..default()
    };
    let right_viewport = Viewport {
        physical_size: UVec2::new(right_width, win_size.y),
        physical_position: UVec2::new(win_size.x - right_width, 0),
        ..default()
    };
    GameLayout { left_viewport, right_viewport }
}
