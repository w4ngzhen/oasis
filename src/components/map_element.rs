use bevy::prelude::*;

#[derive(Component, Copy, Clone, PartialOrd, PartialEq)]
pub enum MapElement {
    Void,
    Wall,
    Floor,
    Custom(i32),
}

#[derive(Component, Copy, Clone, PartialOrd, PartialEq, Default)]
pub struct MapElementProperty {
    is_transparent: bool,
    is_collision: bool,
}

impl MapElementProperty {
    pub fn get(ele: MapElement) -> Self {
        match ele {
            MapElement::Void => {
                Self { is_collision: false, is_transparent: true }
            }
            MapElement::Wall => {
                Self { is_collision: true, is_transparent: false }
            }
            MapElement::Floor => {
                Self { is_collision: false, is_transparent: true }
            }
            _ => Self { is_collision: false, is_transparent: true },
        }
    }
}
