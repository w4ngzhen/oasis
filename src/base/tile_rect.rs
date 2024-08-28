use crate::components::position::Position;
use std::cmp::min;
use std::num;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct TileRect {
    pub x: u64,
    pub y: u64,
    pub w: u64,
    pub h: u64,
}

impl TileRect {
    pub fn new(x: u64, y: u64, w: u64, h: u64) -> Self {
        TileRect { x, y, w, h }
    }

    pub fn new_with_corner(x1: u64, y1: u64, x2: u64, y2: u64) -> Self {
        let left = min(x1, x2);
        let top = min(y1, y2);
        TileRect { x: left, y: top, w: x1.abs_diff(x2), h: y1.abs_diff(y2) }
    }

    pub fn zero() -> Self {
        Self::new(0, 0, 0, 0)
    }

    pub fn left_top(&self) -> Position {
        Position::from([self.x, self.y, 0])
    }

    pub fn right_bottom(&self) -> Position {
        Position::from([self.x + self.w, self.y + self.h, 0])
    }

    pub fn intersect(&self, other: &TileRect) -> bool {
        let x1 = self.x;
        let y1 = self.y;
        let x2 = x1 + self.w;
        let y2 = y1 + self.h;
        let other_x1 = other.x;
        let other_y1 = other.y;
        let other_x2 = other_x1 + other.w;
        let other_y2 = other_y1 + other.h;
        x1 <= other_x2 && x2 >= other_x1 && y1 <= other_y2 && y2 >= other_y1
    }

    pub fn center(&self) -> Position {
        let x1 = self.x;
        let y1 = self.y;
        let x2 = x1 + self.w;
        let y2 = y1 + self.h;
        Position::from([(x1 + x2) / 2, (y1 + y2) / 2, 0])
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(Position),
    {
        for y in self.y..self.y + self.h {
            for x in self.x..self.x + self.h {
                f(Position::new_with_2d(x, y));
            }
        }
    }
}
