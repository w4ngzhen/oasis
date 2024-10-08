use crate::components::field_of_vision::FieldOfVision;
use crate::components::map_element::{MapElement, SystemItemPickCursor};
use crate::components::msg::{MapWantsToPick, WantsToMove};
use crate::components::position_2d::Position2d;
use crate::components::role::Player;
use crate::components::Naming;
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use crate::game_state::InGamingSubState;
use crate::resources::{MapCameraCenter, TileSize};
use bevy::prelude::*;

pub fn player_input(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(&Position2d, Entity), With<Player>>,
    mut next_state: ResMut<NextState<InGamingSubState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyY) {
        // enter explore state.
        next_state.set(InGamingSubState::MapExploring);
        return;
    } else if keyboard_input.just_pressed(KeyCode::KeyP) {
        next_state.set(InGamingSubState::MapPicking);
        return;
    } else if keyboard_input.just_pressed(KeyCode::Period) {
    } else if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(InGamingSubState::Modal);
        return;
    }

    let pressed_key = keyboard_input.get_just_pressed().next().cloned();
    // check game input
    if let Ok((curr_player_pos, player_entity)) = query.get_single() {
        let mut new_pos = curr_player_pos.clone();
        if let Some(key) = pressed_key {
            match key {
                KeyCode::ArrowLeft => new_pos.x -= 1,
                KeyCode::ArrowRight => new_pos.x += 1,
                KeyCode::ArrowUp => new_pos.y -= 1,
                KeyCode::ArrowDown => new_pos.y += 1,
                _ => {}
            }
        }
        if new_pos != *curr_player_pos {
            // 在本系统中，我们仅仅处理玩家输入，不进行移动的操作，
            // 而是产生一个移动组件，在另一个专门处理移动系统中来进行移动
            commands.spawn(WantsToMove {
                entity: player_entity,
                destination: new_pos,
            });
            next_state.set(InGamingSubState::PlayerAction);
        }
    }
}

pub fn setup_map_exploring(
    query: Query<&Position2d, With<Player>>,
    mut map_camera_center: ResMut<MapCameraCenter>,
) {
    if let Ok(player_pos) = query.get_single() {
        map_camera_center.0 = Some(player_pos.clone());
    }
}
pub fn player_explore_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut map_camera_center: ResMut<MapCameraCenter>,
    map_builder: Res<GameMapBuilder>,
    mut next_state: ResMut<NextState<InGamingSubState>>,
) {
    // check free camera
    let pressed_key = keyboard_input.get_just_pressed().next().cloned();
    if let Some(center_pos) = map_camera_center.0 {
        let mut next_pos = center_pos.clone();
        if let Some(key) = pressed_key {
            match key {
                KeyCode::KeyY | KeyCode::Escape => {
                    map_camera_center.0 = None;
                    // back to player turn.
                    next_state.set(InGamingSubState::AwaitingPlayerInput);
                    return;
                }
                KeyCode::ArrowLeft => next_pos.x -= 1,
                KeyCode::ArrowRight => next_pos.x += 1,
                KeyCode::ArrowUp => next_pos.y -= 1,
                KeyCode::ArrowDown => next_pos.y += 1,
                _ => {}
            }
        }
        if map_builder.game_map.in_bounds(&next_pos) {
            map_camera_center.0 = Some(next_pos);
        }
    }
}

pub fn player_picking_input(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    q_fov: Query<&FieldOfVision, With<Player>>,
    mut query_pick_cursor: Query<&mut Position2d, With<SystemItemPickCursor>>,
    mut _map_camera: ResMut<MapCameraCenter>,
    map_builder: Res<GameMapBuilder>,
    mut next_state: ResMut<NextState<InGamingSubState>>,
) {
    if let Ok(mut pick_cursor_pos) = query_pick_cursor.get_single_mut() {
        let pressed_key = keyboard_input.get_just_pressed().next().cloned();
        if let Some(key) = pressed_key {
            let mut next_pick_cursor_pos = pick_cursor_pos.clone();
            match key {
                KeyCode::KeyP | KeyCode::Escape => {
                    next_state.set(InGamingSubState::AwaitingPlayerInput);
                    return;
                }
                KeyCode::ArrowLeft => next_pick_cursor_pos.x -= 1,
                KeyCode::ArrowRight => next_pick_cursor_pos.x += 1,
                KeyCode::ArrowUp => next_pick_cursor_pos.y -= 1,
                KeyCode::ArrowDown => next_pick_cursor_pos.y += 1,
                _ => {}
            }
            let fov = q_fov.single();
            if next_pick_cursor_pos != *pick_cursor_pos
                && map_builder.game_map.in_bounds(&next_pick_cursor_pos)
                && fov.visible_positions.contains(&next_pick_cursor_pos)
            {
                *pick_cursor_pos = next_pick_cursor_pos;
                // generate a pick msg
                commands
                    .spawn((MapWantsToPick { position: next_pick_cursor_pos }));
            }
        }
    }
}

pub fn pick_checking(
    mut commands: Commands,
    q_pick: Query<(Entity, &MapWantsToPick)>,
    q_map_ele: Query<(Entity, &Position2d), With<MapElement>>,
    q_pick_cursor: Query<&SystemItemPickCursor>,
    q_name: Query<&Naming>,
) {
    for (pick_msg, wants_to_pick) in q_pick.iter() {
        info!("check {:?}", wants_to_pick.position);
        let entities = q_map_ele.iter().filter_map(|(ent, pos)| {
            // 注意，PickCursor也是一个MapElement，所以要排除
            if *pos == wants_to_pick.position && !q_pick_cursor.contains(ent) {
                Some(ent)
            } else {
                None
            }
        });
        for ent in entities {
            if let Ok(name) = q_name.get(ent) {
                info!("there is: {}", name.0);
            } else {
                info!("there is something but no name.");
            }
        }
        // info!("nothing!");
        // 清理消息
        commands.entity(pick_msg).despawn_recursive();
    }
}

pub fn scale_map(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut tile_size: ResMut<TileSize>,
) {
    // scale map
    if keyboard_input.just_pressed(KeyCode::Equal) {
        tile_size.0 = (tile_size.0 + 2.).min(32.);
        return;
    }
    if keyboard_input.just_pressed(KeyCode::Minus) {
        tile_size.0 = (tile_size.0 - 2.).max(8.);
        return;
    }
}
