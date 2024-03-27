use crossterm::{
    execute,
    style::{ResetColor, SetForegroundColor, SetBackgroundColor},
    terminal::{Clear, ClearType},
    cursor::{MoveTo, position},
};
pub use crossterm::style::Color;

pub fn set_cursor(x: u16, y: u16) {
    execute!(std::io::stdout(), MoveTo(x, y)).expect("Failed to set cursor position");
}

pub fn get_cursor() -> Pos {
    let (x, y) = match position() {
        Ok(pos) => pos,
        Err(_) => (0, 0),
    };
    return Pos {x: x, y: y};
}

pub struct Pos {
    pub x: u16,
    pub y: u16,
}

pub fn set_color(color: Color) {
    execute!(std::io::stdout(), SetForegroundColor(color)).expect("Failed to set color");
}

pub fn set_background(color: Color) {
    execute!(std::io::stdout(), SetBackgroundColor(color)).expect("Failed to set background");
}

pub fn reset_color() {
    execute!(std::io::stdout(), ResetColor).expect("Failed to reset color");
}

pub fn clear() {
    execute!(std::io::stdout(), Clear(ClearType::All)).expect("Failed to clear terminal");
    set_cursor(0, 0);
}
