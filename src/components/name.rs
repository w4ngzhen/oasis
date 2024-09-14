use bevy::prelude::*;

#[derive(Component, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Naming(pub(crate) String);
