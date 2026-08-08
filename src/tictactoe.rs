use std::collections::HashMap;

use macroquad::prelude::*;

fn draw_cross(x: f32, y: f32, size: f32, thickness: f32, color: Color) {
    let size = size / 2.0;
    draw_line(x - size, y - size, x + size, y + size, thickness, color);
    draw_line(x + size, y - size, x - size, y + size, thickness, color);
}

pub enum BlockState {
    Empty,
    Cross,
    Circle,
}
pub struct Board {
    x: f32,
    y: f32,
    size: f32,
    pub state: HashMap<(u8, u8), BlockState>,
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
    fn get_block_centers(&self, x: u8, y: u8) -> (f32, f32) {
        let top_left_block: (f32, f32) = (self.x + self.size / 6.0, self.y + self.size / 6.0);
        (
            (x - 1) as f32 * self.size / 3.0 + top_left_block.0,
            (y - 1) as f32 * self.size / 3.0 + top_left_block.1,
        )
    }
    pub fn draw(&self) {
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
            self.y + self.size / 3.0 * 2.0,
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
            self.x + self.size / 3.0 * 2.0,
            self.y + self.size,
            2.0,
            DARKGRAY,
        );

        for block in &self.state {
            match block.1 {
                BlockState::Circle => {
                    let (circle_x, circle_y) = self.get_block_centers(block.0.0, block.0.1);
                    draw_circle_lines(circle_x, circle_y, self.size / 6.0 - 10.0, 5.0, DARKBLUE);
                }
                BlockState::Cross => {
                    let (x, y) = self.get_block_centers(block.0.0, block.0.1);
                    draw_cross(x, y, self.size / 3.0 - 10.0, 5.0, RED);
                }
                BlockState::Empty => {}
            }
        }
    }
}
