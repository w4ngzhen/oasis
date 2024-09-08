use crate::components::position_2d::Position2d;
use crate::components::DestroyObject;
use crate::core_module::game_map::game_map_builder::GameMapBuilder;
use bevy::log::info;
use bevy::prelude::*;
use bevy::utils::warn;

pub fn handle_object_destroy(
    mut commands: Commands,
    mut mb: ResMut<GameMapBuilder>,
    q_destroy: Query<(Entity, &DestroyObject)>,
) {
    for (msg, destroy) in q_destroy.iter() {
        let entity = destroy.0;
        commands.entity(entity).despawn_recursive();
        mb.game_map.remove_entity(entity);
        info!("{:?} died.", entity);
        commands.entity(msg).despawn_recursive();
    }
}
