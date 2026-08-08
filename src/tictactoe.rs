use std::collections::HashMap;

use macroquad::prelude::*;

fn draw_cross(x: f32, y: f32, size: f32, thickness: f32, color: Color) {
    let size = size / 2.0;
    draw_line(x - size, y - size, x + size, y + size, thickness, color);
    draw_line(x + size, y - size, x - size, y + size, thickness, color);
}

#[derive(Debug, Copy, Clone)]
pub enum CellState {
    Empty,
    Cross,
    Circle,
}
pub struct Board {
    x: f32,
    y: f32,
    size: f32,
    state: HashMap<(u8, u8), CellState>,
}

impl Board {
    pub fn new(x: f32, y: f32, size: f32) -> Board {
        let mut temp: HashMap<(u8, u8), CellState> = HashMap::new();
        for i in 1..4 {
            for j in 1..4 {
                temp.insert((i, j), CellState::Empty);
            }
        }
        Board {
            x,
            y,
            size,
            state: temp,
        }
    }
    fn get_cell_centers(&self, x: u8, y: u8) -> (f32, f32) {
        let top_left_cell: (f32, f32) = (self.x + self.size / 6.0, self.y + self.size / 6.0);
        (
            (x - 1) as f32 * self.size / 3.0 + top_left_cell.0,
            (y - 1) as f32 * self.size / 3.0 + top_left_cell.1,
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

        for (&(cell_x, cell_y), cell_state) in &self.state {
            match cell_state {
                CellState::Circle => {
                    let (circle_x, circle_y) = self.get_cell_centers(cell_x, cell_y);
                    draw_circle_lines(circle_x, circle_y, self.size / 6.0 - 10.0, 5.0, DARKBLUE);
                }
                CellState::Cross => {
                    let (cross_x, cross_y) = self.get_cell_centers(cell_x, cell_y);
                    draw_cross(cross_x, cross_y, self.size / 3.0 - 10.0, 5.0, RED);
                }
                CellState::Empty => {}
            }
        }
    }
    pub fn update_cell(cell: (u8, u8), state: CellState) {}
}
