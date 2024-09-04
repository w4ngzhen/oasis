use std::ops::Deref;
use crate::components::position::Position;
use crate::components::role::Player;
use crate::components::Movement;
use crate::resources::MapRenderCenterPosition;
use bevy::prelude::*;

/// 该系统仅处理用户输入
pub fn player_input(
    mut commands: Commands,
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    query: Query<(&Position, Entity), With<Player>>,
    mut map_render_center_position: ResMut<MapRenderCenterPosition>,
) {
    let key = keyboard_input.get_pressed().next().cloned();
    if let Some(center_pos) = map_render_center_position.0 {
        let mut next_pos = center_pos.clone();
        if let Some(key) = key {
            match key {
                KeyCode::ArrowLeft => next_pos.x -= 1,
                KeyCode::ArrowRight => next_pos.x += 1,
                KeyCode::ArrowUp => next_pos.y -= 1,
                KeyCode::ArrowDown => next_pos.y += 1,
                _ => {}
            }
        }
        map_render_center_position.0 = Some(next_pos);
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
                if map_render_center_position.0 == None {
                    map_render_center_position.0 = Some(curr_player_pos.clone());
                } else {
                    map_render_center_position.0 = None;
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
