use crate::terminal::{hex_bg, print_liner};

pub struct Renderer {}

impl Renderer {
    pub fn draw_rect(&self, x: u16, y: u16, width: u16, height: u16, color: u32) {
        let rl = " ".repeat(width as usize) + "\x1b[0m";
        let rect = (rl + "\n").repeat(height as usize);
        print_liner(x, y, &hex_bg(color), &rect);
    }
}
