use bevy::prelude::*;

#[derive(Debug)]
pub struct Layout {
    pub left: Rect,
    pub right: Rect,
}

/// 根据窗体尺寸，计算各个模块摄像机的viewport
pub fn calc_game_layout(window_size: UVec2, scale: f32) -> Layout {
    let right_width = (120. * scale) as u32;
    let left: Rect =
        Rect::new(0., 0., 0. + (window_size.x - right_width) as f32, 0. + window_size.y as f32);
    let right: Rect = Rect::new(left.width(), 0., window_size.x as f32, 0. + window_size.y as f32);
    Layout { left, right }
}
