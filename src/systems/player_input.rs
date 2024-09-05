use crate::components::position::Position;
use crate::components::role::Player;
use crate::components::Movement;
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use crate::resources::MapCameraCenter;
use bevy::prelude::*;

/// 该系统仅处理用户输入
pub fn player_input(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(&Position, Entity), With<Player>>,
    mut map_camera_center: ResMut<MapCameraCenter>,
    map_builder: Res<GameMapBuilder>,
) {
    let key = keyboard_input.get_just_pressed().next().cloned();
    // map camera lock
    if let Some(center_pos) = map_camera_center.0 {
        let mut next_pos = center_pos.clone();
        if let Some(key) = key {
            match key {
                KeyCode::ArrowLeft => next_pos.x += 1,
                KeyCode::ArrowRight => next_pos.x -= 1,
                KeyCode::ArrowUp => next_pos.y += 1,
                KeyCode::ArrowDown => next_pos.y -= 1,
                KeyCode::KeyY => {
                    map_camera_center.0 = None;
                    return;
                }
                _ => {}
            }
        }
        if map_builder.game_map.in_bounds(&next_pos) {
            map_camera_center.0 = Some(next_pos);
        }
        return;
    }
    let (curr_player_pos, player_entity) = query.single();
    let mut new_pos = curr_player_pos.clone();
    if let Some(key) = key {
        match key {
            KeyCode::ArrowLeft => new_pos.x -= 1,
            KeyCode::ArrowRight => new_pos.x += 1,
            KeyCode::ArrowUp => new_pos.y -= 1,
            KeyCode::ArrowDown => new_pos.y += 1,
            KeyCode::KeyY => {
                if map_camera_center.0 == None {
                    map_camera_center.0 = Some(curr_player_pos.clone());
                }
            }
            _ => {}
        }
    }
    if new_pos != *curr_player_pos {
        // 在本系统中，我们仅仅处理玩家输入，不进行移动的操作，
        // 而是产生一个移动组件，在另一个专门处理移动系统中来进行移动
        commands.spawn(Movement { entity: player_entity, destination: new_pos });
    }
}
