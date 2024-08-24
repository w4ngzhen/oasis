use bevy::prelude::Component;

#[derive(Component, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Name(pub(crate) String);
