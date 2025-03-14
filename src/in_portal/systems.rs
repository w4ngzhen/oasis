use crate::game_state::GameState;
use crate::in_portal::components::OnPortalScreen;
use crate::in_portal::resources::GamePortalTimer;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use std::ops::DerefMut;

pub fn setup_portal(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon: Handle<Image> = asset_server.load("branding/icon.png");
    commands.spawn((OnPortalScreen, Camera2d::default()));
    // Display the logo
    commands
        .spawn((
            OnPortalScreen,
            Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Node { width: Val::Px(200.), ..default() },
                ImageNode { image: icon, ..default() },
            ));
        });
    // Insert the timer as a resource
    commands.insert_resource(GamePortalTimer(Timer::from_seconds(
        1.0,
        TimerMode::Once,
    )));
}

pub fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<GamePortalTimer>,
) {
    if timer.deref_mut().0.tick(time.delta()).finished() {
        info!("countdown!");
        game_state.set(GameState::InMainMenu);
    }
}
