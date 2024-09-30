use crate::components::field_of_vision::FieldOfVision;
use crate::components::item::Item;
use crate::components::map_element::{MapElement, MapElementProp};
use crate::components::msg::{WantsToAttack, WantsToMove};
use crate::components::position_2d::Position2d;
use crate::components::role::{Monster, Player};
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use crate::resources::PlayerSettings;
use bevy::prelude::*;

pub fn player_movement(
    mut commands: Commands,
    player_settings: Res<PlayerSettings>,
    q_movement_msg: Query<(Entity, &WantsToMove)>,
    mut q_player_mover: Query<(Entity, &mut FieldOfVision), With<Player>>,
    mut q_pos: Query<&mut Position2d>,
    q_item: Query<(Entity), With<Item>>,
    q_monster: Query<(Entity), With<Monster>>,
    q_map_ele_prop: Query<(Entity, &MapElementProp), With<MapElement>>,
    mut map: ResMut<GameMapBuilder>,
) {
    for (msg, wants_to_move) in q_movement_msg.iter() {
        let next_pos = wants_to_move.destination;
        if let Ok((player_entity, mut player_fov)) =
            q_player_mover.get_mut(wants_to_move.entity)
        {
            // 首先检查移动是否合理
            // 1. 是否在地图大范围
            let is_inbounds = map.game_map.in_bounds(&next_pos);
            // 2. 将要移动的到的地方是否被其他带有碰撞的物品占据了
            let occupied_entity: Option<Entity> = if let Some(some_occupied) =
                q_map_ele_prop.iter().find_map(|(entity, prop)| {
                    if let Ok(pos) = q_pos.get(entity) {
                        if *pos == next_pos && prop.is_collision {
                            Some(Some(entity))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }) {
                some_occupied
            } else {
                None
            };

            if is_inbounds && occupied_entity.is_none() {
                // 在地图上，将玩家实体移动到新的位置
                if let Ok(mut player_pos) = q_pos.get_mut(player_entity) {
                    player_pos.x = next_pos.x;
                    player_pos.y = next_pos.y;
                    player_fov.is_dirty_data = true;
                }
            } else if let Some(target) = occupied_entity {
                if q_monster.get(target.clone()).is_ok() {
                    // 对怪物产生一次攻击
                    info!("attack monster!");
                    commands.spawn(WantsToAttack {
                        attacker: player_entity,
                        target: Some(target.clone()),
                    });
                } else if q_item.get(target.clone()).is_ok() {
                    // 如果开启了自动拾取，则触发一次PickUp
                    if player_settings.auto_pick {
                        // todo
                        info!("pick up a things");
                    }
                }
            }
        }
        // delete movement Component
        commands.entity(msg).despawn();
    }
}

pub fn monster_movement(
    mut commands: Commands,
    mut map: ResMut<GameMapBuilder>,
    q_movement_msg: Query<(Entity, &WantsToMove)>,
    mut q_pos: Query<&mut Position2d>,
    mut q_monster_mover: Query<(Entity, &mut FieldOfVision), With<Monster>>,
    q_player: Query<Entity, With<Player>>,
    q_map_ele_prop: Query<(Entity, &MapElementProp), With<MapElement>>,
) {
    for (movement_msg, wants_to_move) in q_movement_msg.iter() {
        let next_pos = wants_to_move.destination;
        if let Ok((monster_entity, mut monster_fov)) =
            q_monster_mover.get_mut(wants_to_move.entity)
        {
            // 首先检查移动是否合理
            // 1. 是否在地图大范围
            let is_inbounds = map.game_map.in_bounds(&next_pos);
            // 2. 将要移动的到的地方是否被其他带有碰撞的物品占据了
            let occupied_entity: Option<Entity> = if let Some(some_occupied) =
                q_map_ele_prop.iter().find_map(|(entity, prop)| {
                    if let Ok(pos) = q_pos.get(entity) {
                        if *pos == next_pos && prop.is_collision {
                            Some(Some(entity))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }) {
                some_occupied
            } else {
                None
            };
            // check the movement is valid
            if is_inbounds && occupied_entity.is_none() {
                if let Ok(mut monster_pos) = q_pos.get_mut(monster_entity) {
                    // 在地图上，将monster实体移动到新的位置
                    monster_pos.x = next_pos.x;
                    monster_pos.y = next_pos.y;
                    monster_fov.is_dirty_data = true;
                }
            } else if let Some(target) = occupied_entity {
                if q_player.get(target.clone()).is_ok() {
                    // 对玩家产生一次攻击
                    info!("attack player!");
                    commands.spawn(WantsToAttack {
                        attacker: monster_entity,
                        target: Some(target.clone()),
                    });
                }
            }
        }
        // delete movement Component
        commands.entity(movement_msg).despawn();
    }
}
