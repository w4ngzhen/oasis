use crate::components::GameUiCamera;
use bevy::prelude::*;
use bevy::utils::default;

pub fn spawn_ui(mut commands: Commands) {
    let camera_entity = commands
        .spawn((
            GameUiCamera,
            Camera2dBundle { camera: Camera { order: 999, ..default() }, ..default() },
        ))
        .id();
    commands.spawn((
        TargetCamera(camera_entity),
        NodeBundle {
            style: Style { width: Val::Percent(100.), height: Val::Percent(100.), ..default() },
            background_color: Color::srgb(0.5, 0.5, 0.5).into(),
            ..default()
        },
    ));
}
