use crate::components::GameHudCamera;
use bevy::prelude::*;
use bevy::utils::default;

pub fn spawn_game_hud(mut commands: Commands, query: Query<Entity, With<GameHudCamera>>) {
    let camera_entity = query.single();
    commands.spawn((
        TargetCamera(camera_entity),
        NodeBundle {
            style: Style { width: Val::Percent(100.), height: Val::Percent(100.), ..default() },
            background_color: Color::srgb(0.5, 0.5, 0.5).into(),
            ..default()
        },
    ));
}
