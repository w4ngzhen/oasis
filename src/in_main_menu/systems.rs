use crate::game_state::GameState;
use crate::in_main_menu::components::{InMainMenuScreen, MenuButtonAction};
use bevy::prelude::*;
use std::ops::DerefMut;

pub fn setup_main_menu(mut commands: Commands) {
    commands.spawn((InMainMenuScreen, Camera2dBundle::default()));
    commands
        .spawn((
            InMainMenuScreen,
            NodeBundle {
                style: Style { height: Val::Percent(100.), width: Val::Percent(100.), ..default() },
                background_color: Color::srgb(0.1, 0.1, 0.1).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            // parent.spa
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.),
                        position_type: PositionType::Absolute,
                        left: Val::Px(40.0),
                        bottom: Val::Px(40.0),
                        padding: UiRect::all(Val::Px(10.)),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    border_color: Color::srgb(1.0, 0., 0.).into(),
                    ..default()
                })
                .with_children(|p| {
                    create_menu_button(p, MenuButtonAction::NewGame, "New Game");
                    create_menu_button(p, MenuButtonAction::ContinueGame, "Continue Game");
                    create_menu_button(p, MenuButtonAction::Settings, "Settings");
                    create_menu_button(p, MenuButtonAction::Quit, "Quit");
                });
        });
}

fn create_menu_button(builder: &mut ChildBuilder, action: MenuButtonAction, button_label: &str) {
    let button_text_style = TextStyle { font_size: 40.0, color: BUTTON_NORMAL_COLOR, ..default() };
    builder.spawn((action, ButtonBundle { ..default() })).with_children(|p| {
        p.spawn(TextBundle::from_section(button_label.clone(), button_text_style));
    });
}

pub fn menu_button_system(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
    mut text_query: Query<&mut Text>,
) {
    fn set_text_color(text: &mut Text, color: Color) {
        text.sections.iter_mut().for_each(|mut section| {
            section.style.color = color;
        })
    }
    for (interaction, action, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                set_text_color(&mut text, BUTTON_PRESSED_COLOR);
                match action {
                    MenuButtonAction::NewGame => {
                        game_state.deref_mut().set(GameState::InPlayerConfig);
                    }
                    MenuButtonAction::ContinueGame => {
                        game_state.deref_mut().set(GameState::InGaming);
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
                set_text_color(&mut text, BUTTON_HOVER_COLOR);
            }
            Interaction::None => {
                set_text_color(&mut text, BUTTON_NORMAL_COLOR);
            }
        }
    }
}

pub fn menu_button_action_system() {}

const BUTTON_NORMAL_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
const BUTTON_PRESSED_COLOR: Color = Color::srgb(1., 1., 1.);
const BUTTON_HOVER_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
