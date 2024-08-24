use crate::game_state::GameState;
use crate::in_player_config::components::InPlayerConfigScreen;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

pub fn setup_player_config_screen(mut commands: Commands) {
    commands.spawn((InPlayerConfigScreen, Camera2dBundle::default()));
    commands
        .spawn((
            InPlayerConfigScreen,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::srgb(1.0, 0.5, 0.5).into(),
                ..default()
            },
        ))
        .with_children(|p| {
            p.spawn(TextBundle::from_section(
                "hello, player config.",
                TextStyle { color: Color::srgb(1., 1., 1.), ..default() },
            ));
        });
}

pub fn player_config_systems() {}

pub fn play_config_keyboard_handle(
    mut event_reader: EventReader<KeyboardInput>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for key_board_input in event_reader.read() {
        if key_board_input.key_code == KeyCode::Escape {
            game_state.set(GameState::InMainMenu);
        }
    }
}
