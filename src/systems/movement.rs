use crate::components::position::Position;
use crate::components::Movement;
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use bevy::prelude::*;

pub fn movement(
    mut commands: Commands,
    query_movement: Query<(Entity, &Movement)>,
    mut query_mover: Query<(Entity, &mut Position)>,
    map: Res<GameMapBuilder>,
) {
    for (movement_entity, movement) in query_movement.iter() {
        let new_dest = movement.destination;
        if let Ok((_mover_entity, mut mover_curr_pos)) = query_mover.get_mut(movement.entity) {
            // check the movement is valid
            if map.game_map.in_bounds(&new_dest) {
                mover_curr_pos.x = new_dest.x;
                mover_curr_pos.y = new_dest.y;
            }
        }
        // delete movement Component
        commands.entity(movement_entity).despawn();
    }
}
