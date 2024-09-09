use crate::components::attack::Attack;
use crate::components::field_of_vision::FieldOfVision;
use crate::components::position_2d::Position2d;
use crate::components::role::{Monster, Player};
use crate::components::Movement;
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use bevy::prelude::*;

pub fn movement(
    mut commands: Commands,
    query_movement: Query<(Entity, &Movement)>,
    mut query_mover: Query<(Entity, &mut FieldOfVision, &mut Position2d)>,
    mut map: ResMut<GameMapBuilder>,
) {
    for (movement_entity, movement) in query_movement.iter() {
        info!("movement {:?}", movement);
        let new_dest = movement.destination;
        if let Ok((mover_entity, mut mover_fov, mut mover_curr_pos)) =
            query_mover.get_mut(movement.entity)
        {
            // check the movement is valid
            if map.game_map.in_bounds(&new_dest)
                && !map.game_map.is_tile_opacity(&new_dest)
                && map.game_map.occupation.get(&new_dest).is_none()
            {
                // 准备完成移动
                // 先更新map中的entity的occupation数据
                if map.game_map.occupation.get(&mover_curr_pos) == Some(&mover_entity) {
                    map.game_map.occupation.remove(&mover_curr_pos);
                    map.game_map.occupation.insert(new_dest.clone(), mover_entity);
                }
                mover_curr_pos.x = new_dest.x;
                mover_curr_pos.y = new_dest.y;
                mover_fov.is_dirty_data = true;
            } else if let Some(target) = map.game_map.occupation.get(&new_dest) {
                // 产生一次攻击
                info!("attack");
                commands.spawn(Attack { attacker: mover_entity, target: Some(target.clone()) });
            }
        }
        // delete movement Component
        commands.entity(movement_entity).despawn();
    }
}
