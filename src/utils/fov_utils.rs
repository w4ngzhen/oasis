use crate::components::position_2d::Position2d;
use doryen_fov::{FovAlgorithm, FovRecursiveShadowCasting, MapData};
use std::cmp::{max, min};
use std::collections::HashSet;

/// 视野范围、地图信息，计算出能够看到的位置
pub fn calc_fov<F>(
    observer_pos: &Position2d,
    range: i32,
    map_size: (i32, i32),
    check_is_block_view: F,
) -> HashSet<Position2d>
where
    F: Fn(i32, i32) -> bool,
{
    let (map_w, map_h) = map_size;
    let x_range =
        (max(0, observer_pos.x - range), min(observer_pos.x + range, map_w));
    let y_range =
        (max(0, observer_pos.y - range), min(observer_pos.y + range, map_h));
    let view_rect_w = x_range.1 - x_range.0;
    let view_rect_h = y_range.1 - y_range.0;
    let mut view_map_data =
        MapData::new(view_rect_w as usize, view_rect_h as usize);
    for origin_x in x_range.0..x_range.1 {
        for origin_y in y_range.0..y_range.1 {
            if check_is_block_view(origin_x, origin_y) {
                let offset_x = origin_x - x_range.0;
                let offset_y = origin_y - y_range.0;
                view_map_data.set_transparent(
                    offset_x as usize,
                    offset_y as usize,
                    false,
                );
            }
        }
    }
    let mut fov = FovRecursiveShadowCasting::new();
    let role_offset_x = observer_pos.x - x_range.0;
    let role_offset_y = observer_pos.y - y_range.0;
    fov.compute_fov(
        &mut view_map_data,
        role_offset_x as usize,
        role_offset_y as usize,
        range as usize,
        true,
    );
    let mut visible_positions: HashSet<Position2d> = HashSet::new();
    for origin_x in x_range.0..x_range.1 {
        for origin_y in y_range.0..y_range.1 {
            let offset_x = (origin_x - x_range.0) as usize;
            let offset_y = (origin_y - y_range.0) as usize;
            if view_map_data.is_in_fov(offset_x, offset_y) {
                visible_positions.insert(Position2d::from([origin_x, origin_y]));
            }
        }
    }
    visible_positions
}
