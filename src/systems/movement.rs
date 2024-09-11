use crate::components::attack::Attack;
use crate::components::field_of_vision::FieldOfVision;
use crate::components::position_2d::Position2d;
use crate::components::role::{Monster, Player};
use crate::components::WantsToMove;
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use bevy::prelude::*;

pub fn movement(
    mut commands: Commands,
    query_movement: Query<(Entity, &WantsToMove)>,
    mut query_mover: Query<(Entity, &mut FieldOfVision, &mut Position2d)>,
    mut map: ResMut<GameMapBuilder>,
) {
    for (movement_entity, wants_to_move) in query_movement.iter() {
        info!("movement {:?}", wants_to_move);
        let new_dest = wants_to_move.destination;
        if let Ok((mover_entity, mut mover_fov, mut mover_curr_pos)) =
            query_mover.get_mut(wants_to_move.entity)
        {
            // check the movement is valid
            if map.game_map.in_bounds(&new_dest)
                && !map.game_map.is_tile_opacity(&new_dest)
                && map.game_map.occupation.get(&new_dest).is_none()
            {
                // 如果移动的实体是地图上的occupation中的，则需要调整对应的实体位置
                if map.game_map.occupation.get(&mover_curr_pos) == Some(&mover_entity) {
                    if let Some(ent) = map.game_map.occupation.remove(&mover_curr_pos) {
                        map.game_map.occupation.insert(new_dest.clone(), ent);
                    }
                }
                mover_curr_pos.x = new_dest.x;
                mover_curr_pos.y = new_dest.y;
                mover_fov.is_dirty_data = true;
                // map.game_map.print_occupations();
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
