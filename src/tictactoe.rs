use std::collections::HashMap;

use macroquad::prelude::*;

enum BlockState {
    Empty,
    Cross,
    Circle,
}
pub struct Board {
    x: f32,
    y: f32,
    size: f32,
    state: HashMap<(u8, u8), BlockState>,
}

impl Board {
    pub fn new(x: f32, y: f32, size: f32) -> Board {
        Board {
            x,
            y,
            size,
            state: HashMap::new(),
        }
    }
    pub fn draw(&mut self) {
        draw_line(
            self.x,
            self.y + self.size / 3.0,
            self.x + self.size,
            self.y + self.size / 3.0,
            2.0,
            DARKGRAY,
        );
        draw_line(
            self.x,
            self.y + self.size / 3.0 * 2.0,
            self.x + self.size,
            self.y + self.size / 3.0,
            2.0,
            DARKGRAY,
        );
        draw_line(
            self.x + self.size / 3.0,
            self.y,
            self.x + self.size / 3.0,
            self.y + self.size,
            2.0,
            DARKGRAY,
        );
        draw_line(
            self.x + self.size / 3.0 * 2.0,
            self.y,
            self.x + self.size / 3.0,
            self.y + self.size,
            2.0,
            DARKGRAY,
        );
    }
}
