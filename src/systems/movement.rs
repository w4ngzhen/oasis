use crate::components::position::Position;
use crate::components::Movement;
use bevy::prelude::*;

pub fn movement(
    mut commands: Commands,
    query_movement: Query<(Entity, &Movement)>,
    mut query_mover: Query<(Entity, &mut Position)>,
) {
    for (movement_entity, movement) in query_movement.iter() {
        let new_dest = movement.destination;
        if let Ok((_mover_entity, mut mover_curr_pos)) = query_mover.get_mut(movement.entity) {
            mover_curr_pos.x = new_dest.x;
            mover_curr_pos.y = new_dest.y;
        }
        // delete movement Component
        commands.entity(movement_entity).despawn();
    }
}
