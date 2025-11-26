use crate::terminal::{hex_bg, print_liner};

#[derive(Debug, Clone)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

pub struct Renderer {
    pos: Point,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            pos: Point { x: 0, y: 0 },
        }
    }

    pub fn draw_rect(&self, x: u16, y: u16, width: u16, height: u16, color: u32) {
        let rl = " ".repeat(width as usize) + "\x1b[0m";
        let rect = (rl + "\n").repeat(height as usize);
        print_liner(self.pos.x + x, self.pos.y + y, &hex_bg(color), &rect);
    }

    pub fn set_position(&mut self, x: u16, y: u16) {
        self.pos.x = x;
        self.pos.y = y;
    }

    pub fn set_position_point(&mut self, pos: Point) {
        self.pos = pos;
    }
}
