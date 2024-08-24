use bevy::prelude::Component;

#[derive(Component)]
pub struct InMainMenuScreen;

#[derive(Component)]
pub enum MenuButtonAction {
    NewGame,
    ContinueGame,
    Settings,
    Quit,
}
