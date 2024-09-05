use crate::components::position::Position;
use std::cmp::min;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct TileRect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl TileRect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        TileRect { x, y, w, h }
    }

    pub fn new_with_corner(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        let left = min(x1, x2);
        let top = min(y1, y2);
        TileRect { x: left, y: top, w: x1.abs_diff(x2) as i32, h: y1.abs_diff(y2) as i32 }
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
