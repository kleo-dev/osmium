use std::fmt::Write as _;
use std::io::{self, Write};

use crossterm::{ExecutableCommand, terminal};

pub fn flush() -> io::Result<()> {
    std::io::stdout().flush()
}

pub fn clear() -> std::io::Result<()> {
    std::io::stdout().execute(terminal::Clear(terminal::ClearType::All))?;
    std::io::stdout().execute(terminal::Clear(terminal::ClearType::Purge))?;
    Ok(())
}

pub fn hide_cursor() -> io::Result<()> {
    print!("\x1b[?25l");
    flush()
}

pub fn show_cursor() -> io::Result<()> {
    print!("\x1B[?25h");
    flush()
}

pub fn write_liner(buf: &mut String, x: u16, y: u16, liner: &str, text: &str) {
    for (i, line) in text.lines().enumerate() {
        write!(
            buf,
            "\x1b[{};{}H{liner}{line}\x1b[0m",
            (i as u16) + y + 1,
            x + 1
        )
        .unwrap();
    }
}

pub fn hex(hex: u32) -> String {
    let r = ((hex >> 16) & 0xFF) as u8;
    let g = ((hex >> 8) & 0xFF) as u8;
    let b = (hex & 0xFF) as u8;
    format!("\x1b[38;2;{r};{g};{b}m")
}

pub fn hex_bg(hex: u32) -> String {
    let r = ((hex >> 16) & 0xFF) as u8;
    let g = ((hex >> 8) & 0xFF) as u8;
    let b = (hex & 0xFF) as u8;
    format!("\x1b[48;2;{r};{g};{b}m")
}
