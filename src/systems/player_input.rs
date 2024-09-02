use crate::components::position::Position;
use crate::components::role::Player;
use crate::components::Movement;
use bevy::prelude::*;

/// 该系统仅处理用户输入
pub fn player_input(
    mut commands: Commands,
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    query: Query<(&Position, Entity), With<Player>>,
) {
    let (old_pos, player_entity) = query.single();
    let mut new_pos = old_pos.clone();
    let key = keyboard_input.get_pressed().next().cloned();
    if let Some(key) = key {
        match key {
            KeyCode::ArrowLeft => new_pos.x -= 1,
            KeyCode::ArrowRight => new_pos.x += 1,
            KeyCode::ArrowUp => new_pos.y -= 1,
            KeyCode::ArrowDown => new_pos.y += 1,
            _ => {}
        }
    }
    if new_pos != *old_pos {
        // 在本系统中，我们仅仅处理玩家输入，不进行移动的操作，
        // 而是产生一个移动组件，在另一个专门处理移动系统中来进行移动
        commands.spawn(Movement { entity: player_entity, destination: new_pos });
    }
}
