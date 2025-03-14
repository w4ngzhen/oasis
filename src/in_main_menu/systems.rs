use crate::game_state::GameState;
use crate::in_main_menu::components::{InMainMenuScreen, MenuButtonAction};
use bevy::prelude::*;
use std::ops::DerefMut;

pub fn setup_main_menu(mut commands: Commands) {
    commands.spawn((InMainMenuScreen, Camera2d::default()));
    commands
        .spawn((
            InMainMenuScreen,
            Node {
                height: Val::Percent(100.),
                width: Val::Percent(100.),
                ..Default::default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1).into()),
        ))
        .with_children(|parent| {
            // parent.spa
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.),
                        position_type: PositionType::Absolute,
                        left: Val::Px(40.0),
                        bottom: Val::Px(40.0),
                        padding: UiRect::all(Val::Px(10.)),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(1.0, 0., 0.).into()),
                ))
                .with_children(|parent_builder| {
                    create_menu_button(
                        parent_builder,
                        MenuButtonAction::NewGame,
                        "New Game",
                    );
                    create_menu_button(
                        parent_builder,
                        MenuButtonAction::ContinueGame,
                        "Continue Game",
                    );
                    create_menu_button(
                        parent_builder,
                        MenuButtonAction::Settings,
                        "Settings",
                    );
                    create_menu_button(
                        parent_builder,
                        MenuButtonAction::Quit,
                        "Quit",
                    );
                });
        });
}

fn create_menu_button(
    builder: &mut ChildBuilder,
    btn_action: MenuButtonAction,
    button_label: &str,
) {
    builder.spawn((btn_action, Button { ..default() })).with_children(|p| {
        p.spawn((
            Text::new(button_label),
            TextFont { font_size: 40.0, ..default() },
            TextColor(BUTTON_NORMAL_COLOR),
        ));
    });
}

pub fn menu_button_system(
    mut game_state: ResMut<NextState<GameState>>,
    mut q_interaction: Query<
        (&Interaction, &MenuButtonAction, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut q_text_color: Query<&mut TextColor, With<Text>>,
) {
    fn set_text_color(text_color: &mut TextColor, color: Color) {
        *text_color = color.into();
    }
    for (interaction, action, children) in &mut q_interaction {
        let mut text_color = q_text_color.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                set_text_color(&mut text_color, BUTTON_PRESSED_COLOR);
                match action {
                    MenuButtonAction::NewGame => {
                        game_state.deref_mut().set(GameState::InPlayerConfig);
                    }
                    MenuButtonAction::ContinueGame => {
                        game_state.deref_mut().set(GameState::PrepareGame);
                    }
                    MenuButtonAction::Settings => {
                        game_state.deref_mut().set(GameState::InGameSetting);
                    }
                    MenuButtonAction::Quit => {
                        game_state.deref_mut().set(GameState::InGameSetting);
                    }
                    _ => {}
                }
            }
            Interaction::Hovered => {
                set_text_color(&mut text_color, BUTTON_HOVER_COLOR);
            }
            Interaction::None => {
                set_text_color(&mut text_color, BUTTON_NORMAL_COLOR);
            }
        }
    }
}

pub fn menu_button_action_system() {}

const BUTTON_NORMAL_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
const BUTTON_PRESSED_COLOR: Color = Color::srgb(1., 1., 1.);
const BUTTON_HOVER_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
