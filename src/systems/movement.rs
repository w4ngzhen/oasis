use crate::components::attack::WantsToAttack;
use crate::components::field_of_vision::FieldOfVision;
use crate::components::item::Item;
use crate::components::position_2d::Position2d;
use crate::components::role::{Monster, Player};
use crate::components::WantsToMove;
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use crate::resources::PlayerSettings;
use bevy::prelude::*;

pub fn player_movement(
    mut commands: Commands,
    player_settings: Res<PlayerSettings>,
    q_movement_msg: Query<(Entity, &WantsToMove)>,
    mut q_player_mover: Query<
        (Entity, &mut FieldOfVision, &mut Position2d),
        With<Player>,
    >,
    q_item: Query<(Entity), With<Item>>,
    q_monster: Query<(Entity), With<Monster>>,
    mut map: ResMut<GameMapBuilder>,
) {
    for (movement_msg, wants_to_move) in q_movement_msg.iter() {
        let next_pos = wants_to_move.destination;
        if let Ok((player_entity, mut player_fov, mut player_pos)) =
            q_player_mover.get_mut(wants_to_move.entity)
        {
            // check the movement is valid
            if map.game_map.in_bounds(&next_pos)
                && !map.game_map.is_tile_opacity(&next_pos)
                && map.game_map.occupation.get(&next_pos).is_none()
            {
                // 在地图上，将玩家实体移动到新的位置
                map.game_map.move_entity(player_entity, next_pos);
                // 如果正在移动的实体是地图上的occupation中的，则需要调整对应的位置
                player_pos.x = next_pos.x;
                player_pos.y = next_pos.y;
                player_fov.is_dirty_data = true;
            } else if let Some(target) = map.game_map.occupation.get(&next_pos)
            {
                if q_monster.get(target.clone()).is_ok() {
                    // 对怪物产生一次攻击
                    info!("attack");
                    commands.spawn(WantsToAttack {
                        attacker: player_entity,
                        target: Some(target.clone()),
                    });
                } else if q_item.get(target.clone()).is_ok() {
                    // 如果开启了自动拾取，则触发一次PickUp
                    if player_settings.auto_pick {
                        info!("pick up a things");
                    }
                }
            }
        }
        // delete movement Component
        commands.entity(movement_msg).despawn();
    }
}

pub fn monster_movement(
    mut commands: Commands,
    mut map: ResMut<GameMapBuilder>,
    q_movement_msg: Query<(Entity, &WantsToMove)>,
    mut q_monster_mover: Query<
        (Entity, &mut FieldOfVision, &mut Position2d),
        With<Monster>,
    >,
    q_player: Query<Entity, With<Player>>,
) {
    for (movement_msg, wants_to_move) in q_movement_msg.iter() {
        let next_pos = wants_to_move.destination;
        if let Ok((monster_entity, mut monster_fov, mut monster_pos)) =
            q_monster_mover.get_mut(wants_to_move.entity)
        {
            // check the movement is valid
            if map.game_map.in_bounds(&next_pos)
                && !map.game_map.is_tile_opacity(&next_pos)
                && map.game_map.occupation.get(&next_pos).is_none()
            {
                // 在地图上，将玩家实体移动到新的位置
                map.game_map.move_entity(monster_entity, next_pos);
                // 如果正在移动的实体是地图上的occupation中的，则需要调整对应的位置
                monster_pos.x = next_pos.x;
                monster_pos.y = next_pos.y;
                monster_fov.is_dirty_data = true;
            } else if let Some(target) = map.game_map.occupation.get(&next_pos)
            {
                if q_player.get(target.clone()).is_ok() {
                    // 对怪物产生一次攻击
                    info!("attack");
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
