use crate::terminal::{self, hex_bg, write_liner};

#[derive(Debug, Clone)]
pub struct Point<I = u16> {
    pub x: I,
    pub y: I,
}

pub struct Renderer {
    pos: Point,
    buf: String,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            pos: Point { x: 0, y: 0 },
            buf: String::new(),
        }
    }

    pub fn draw_rect(&mut self, x: u16, y: u16, width: u16, height: u16, color: u32) {
        let rl = " ".repeat(width as usize) + "\x1b[0m";
        let rect = (rl + "\n").repeat(height as usize);
        write_liner(
            &mut self.buf,
            self.pos.x + x,
            self.pos.y + y,
            &hex_bg(color),
            &rect,
        );
    }

    pub fn draw_pixel(&mut self, x: u16, y: u16, color: u32) {
        write_liner(
            &mut self.buf,
            self.pos.x + x,
            self.pos.y + y,
            &hex_bg(color),
            &" \x1b[0m",
        );
    }

    pub fn draw_line(&mut self, x0: i16, y0: i16, x1: i16, y1: i16, color: u32) {
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        let (mut x, mut y) = (x0, y0);

        loop {
            self.draw_pixel(x as u16, y as u16, color);

            if x == x1 && y == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn draw_vertex_outline(&mut self, x: u16, y: u16, vertices: &[Point], color: u32) {
        if vertices.len() < 2 {
            return;
        }

        for i in 0..vertices.len() {
            let p0 = &vertices[i];
            let p1 = &vertices[(i + 1) % vertices.len()];

            self.draw_line(
                x as i16 + p0.x as i16,
                y as i16 + p0.y as i16,
                x as i16 + p1.x as i16,
                y as i16 + p1.y as i16,
                color,
            );
        }
    }

    pub fn draw_vertex(&mut self, x: u16, y: u16, vertices: &[Point], color: u32) {
        if vertices.len() < 3 {
            return;
        }

        self.draw_vertex_outline(x, y, vertices, color);

        let pts: Vec<(i32, i32)> = vertices
            .iter()
            .map(|p| ((p.x + x) as i32, (p.y + y) as i32))
            .collect();

        let min_y = pts.iter().map(|(_, y)| *y).min().unwrap();
        let max_y = pts.iter().map(|(_, y)| *y).max().unwrap();

        for y in min_y..=max_y {
            let mut intersections = Vec::<i32>::new();

            for i in 0..pts.len() {
                let (x1, y1) = pts[i];
                let (x2, y2) = pts[(i + 1) % pts.len()];

                // same edge test as your original
                if (y1 <= y && y2 > y) || (y2 <= y && y1 > y) {
                    let dy = y2 - y1;
                    let dx = x2 - x1;

                    let t = (y - y1) as f32 / dy as f32;
                    let x = (x1 as f32 + t * dx as f32).round() as i32;
                    intersections.push(x);
                }
            }

            if intersections.len() < 2 {
                continue;
            }

            intersections.sort_unstable();

            // fill between pairs
            for pair in intersections.chunks(2) {
                if pair.len() < 2 {
                    continue;
                }
                for x in pair[0]..=pair[1] {
                    self.draw_pixel(x as u16, y as u16, color);
                }
            }
        }
    }

    pub fn render(&self) {
        print!("{}", self.buf);
        terminal::flush().unwrap();
    }

    pub fn set_position(&mut self, x: u16, y: u16) {
        self.pos.x = x;
        self.pos.y = y;
    }

    pub fn set_position_point(&mut self, pos: Point) {
        self.pos = pos;
    }
}
