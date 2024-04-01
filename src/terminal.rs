pub use crossterm::event::KeyCode;
pub use crossterm::event::KeyEventKind;
pub use crossterm::style::Color;
use crossterm::{
    cursor::{position, Hide, MoveTo, SetCursorStyle, Show},
    event::{self, Event, KeyEvent},
    execute,
    style::{ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};

pub fn set_cursor(pos: Pos) {
    execute!(std::io::stdout(), MoveTo(pos.x, pos.y)).expect("Failed to set cursor position");
}

pub fn enable() {
    enable_raw_mode().expect("Failed to enable raw mode");
}

pub fn disable() {
    disable_raw_mode().expect("Failed to disable raw mode");
}

pub fn get_cursor() -> Pos {
    let (x, y) = match position() {
        Ok(pos) => pos,
        Err(_) => (0, 0),
    };
    return Pos { x, y };
}

pub fn clear_line(line: u16) {
    clear_lines(line, line);
}

pub struct Pos {
    pub x: u16,
    pub y: u16,
}

impl Pos {
    pub fn from(x: u16, y: u16) -> Self {
        return Self { x, y };
    }
    pub fn from_tuple((x, y): (u16, u16)) -> Self {
        return Self::from(x, y);
    }
    pub fn to_tuple(&self) -> (u16, u16) {
        return (self.x, self.y);
    }
    pub fn set(&mut self, pos: Self) -> &Self {
        self.x = pos.x;
        self.y = pos.y;
        return self;
    }
    pub fn to_pos(&self) -> Self {
        return Self {
            x: self.x,
            y: self.y,
        };
    }
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
    clear_lines(0, term_size().1);
    set_cursor(Pos::from(0, 0));
}

fn clear_lines(start_y: u16, end_y: u16) {
    for y in start_y..=end_y {
        set_cursor(Pos::from(0, y));
        print!("{}[K", 27 as char);
    }
}

pub struct Key {
    pub kind: KeyEventKind,
    pub key: KeyCode,
}

impl Key {
    pub fn filter(&self, filter: KeyEventKind) -> Option<&Self> {
        if self.kind == filter {
            return None;
        }
        return Some(self);
    }
    pub fn only(&self, filter: KeyEventKind) -> Option<&Self> {
        if self.kind == filter {
            return Some(self);
        }
        return None;
    }
}

pub fn key() -> Option<Key> {
    if let Event::Key(KeyEvent { code, kind, .. }) = event::read().expect("Failed to read event") {
        return Some(Key { kind, key: code });
    } else {
        return None;
    }
}

pub enum CursorStyle {
    DefaultUserShape,
    BlinkingBlock,
    SteadyBlock,
    BlinkingUnderScore,
    SteadyUnderScore,
    BlinkingBar,
    SteadyBar,
}

impl CursorStyle {
    pub fn to_cursor_style(&self) -> SetCursorStyle {
        match self {
            Self::DefaultUserShape => SetCursorStyle::DefaultUserShape,
            Self::BlinkingBlock => SetCursorStyle::BlinkingBlock,
            Self::SteadyBlock => SetCursorStyle::SteadyBlock,
            Self::BlinkingUnderScore => SetCursorStyle::BlinkingUnderScore,
            Self::SteadyUnderScore => SetCursorStyle::SteadyUnderScore,
            Self::BlinkingBar => SetCursorStyle::BlinkingBar,
            Self::SteadyBar => SetCursorStyle::SteadyBar,
        }
    }
}

pub fn set_cursor_style(style: CursorStyle) {
    execute!(std::io::stdout(), style.to_cursor_style()).expect("Failed to set the Cursor");
}

pub fn term_size() -> (u16, u16) {
    if let Ok((width, height)) = terminal::size() {
        (width, height)
    } else {
        (0, 0)
    }
}

pub fn hide_cursor() {
    execute!(std::io::stdout(), Hide).expect("Failed to hide the Cursor");
}

pub fn show_cursor() {
    execute!(std::io::stdout(), Show).expect("Failed to hide the Cursor");
}

pub fn use_alt() {
    execute!(std::io::stdout(), EnterAlternateScreen).expect("Failed to switch to alt screen");
}

pub fn use_main() {
    execute!(std::io::stdout(), LeaveAlternateScreen).expect("Failed to switch to main screen");
}
