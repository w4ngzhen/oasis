use crate::components::position::Position;

pub struct FieldOfVision {
    pub visible_tiles: Vec<Position>,
    pub range: u64,
}
