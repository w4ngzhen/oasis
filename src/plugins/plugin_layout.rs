use crate::systems::layout::camera_layout;
use bevy::prelude::*;

pub struct LayoutPlugin;

impl Plugin for LayoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_layout);
    }
}
