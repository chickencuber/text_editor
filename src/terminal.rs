use std::vec;

pub use crossterm::event::KeyCode;
pub use crossterm::event::KeyEventKind;
pub use crossterm::style::Color;

//this is the previous version of this file, put in a private module
mod helper {
    use crossterm::event::KeyCode;
    use crossterm::event::KeyEventKind;
    use crossterm::style::Color;
    use crossterm::{
        cursor::{position, Hide, MoveTo, SetCursorStyle, Show},
        event::{self, Event, KeyEvent},
        execute, queue,
        style::{ResetColor, SetBackgroundColor, SetForegroundColor},
        terminal::{
            self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
    };

    pub static mut AUTO_FLUSH: bool = false;

    pub fn set_cursor(pos: Pos) {
        if unsafe { AUTO_FLUSH } {
            execute!(std::io::stdout(), MoveTo(pos.x, pos.y))
                .expect("Failed to set cursor position");
            return;
        }
        queue!(std::io::stdout(), MoveTo(pos.x, pos.y)).expect("Failed to set cursor position");
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
        pub fn clone(&self) -> Self {
            return Self {
                x: self.x,
                y: self.y,
            };
        }
    }

    pub fn set_color(color: Color) {
        if unsafe { AUTO_FLUSH } {
            execute!(std::io::stdout(), SetForegroundColor(color)).expect("Failed to set color");
            return;
        }
        queue!(std::io::stdout(), SetForegroundColor(color)).expect("Failed to set color");
    }

    pub fn set_background(color: Color) {
        if unsafe { AUTO_FLUSH } {
            execute!(std::io::stdout(), SetBackgroundColor(color))
                .expect("Failed to set background");
            return;
        }
        queue!(std::io::stdout(), SetBackgroundColor(color)).expect("Failed to set background");
    }

    pub fn reset_color() {
        if unsafe { AUTO_FLUSH } {
            execute!(std::io::stdout(), ResetColor).expect("Failed to reset color");
            return;
        }
        queue!(std::io::stdout(), ResetColor).expect("Failed to reset color");
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
        if let Event::Key(KeyEvent { code, kind, .. }) =
            event::read().expect("Failed to read event")
        {
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
        if unsafe { AUTO_FLUSH } {
            execute!(std::io::stdout(), style.to_cursor_style()).expect("Failed to set the Cursor");
            return;
        }
        queue!(std::io::stdout(), style.to_cursor_style()).expect("Failed to set the Cursor");
    }

    pub fn term_size() -> (u16, u16) {
        if let Ok((width, height)) = terminal::size() {
            (width, height)
        } else {
            (0, 0)
        }
    }

    pub fn hide_cursor() {
        if unsafe { AUTO_FLUSH } {
            execute!(std::io::stdout(), Hide).expect("Failed to hide the Cursor");
            return;
        }
        queue!(std::io::stdout(), Hide).expect("Failed to hide the Cursor");
    }

    pub fn show_cursor() {
        if unsafe { AUTO_FLUSH } {
            execute!(std::io::stdout(), Show).expect("Failed to show the Cursor");
            return;
        }
        queue!(std::io::stdout(), Show).expect("Failed to show the Cursor");
    }

    pub fn use_alt() {
        if unsafe { AUTO_FLUSH } {
            execute!(std::io::stdout(), EnterAlternateScreen)
                .expect("Failed to switch to alt screen");
            return;
        }
        queue!(std::io::stdout(), EnterAlternateScreen).expect("Failed to switch to alt screen");
    }

    pub fn use_main() {
        if unsafe { AUTO_FLUSH } {
            execute!(std::io::stdout(), LeaveAlternateScreen)
                .expect("Failed to switch to main screen");
            return;
        }
        queue!(std::io::stdout(), LeaveAlternateScreen).expect("Failed to switch to main screen");
    }

    pub fn flush() {
        std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush");
    }
}

use helper::flush;

pub use helper::{CursorStyle, Key, Pos};

pub struct Colors {
    pub color: Color,
    pub text: char,
}

pub struct Renderable_Colors {
    pub color: Color,
    pub text: String,
}

pub struct Buffer {
    pub buf: Vec<Vec<Colors>>,
    pub pos: Pos,
    pub color: Color,
}

impl Buffer {
    pub fn new() -> Self {
        return Self {
            buf: vec![vec![]],
            pos: Pos::from(0, 0),
            color: Color::White,
        };
    }
    pub fn char_at_pos(&mut self, char: char) -> &mut Self {
        while self.pos.y >= self.buf.len().try_into().unwrap() {
            self.buf.push(Vec::new());
        }
        while self.pos.x >= self.buf[self.pos.y as usize].len().try_into().unwrap() {
            self.buf[self.pos.y as usize].push(Colors {
                color: Color::White,
                text: ' ',
            });
        }
        if char == '\n' {
            self.pos.y += 1;
            self.pos.x = 0;
            return self;
        }
        self.buf[self.pos.y as usize][self.pos.x as usize].text = char;
        self.buf[self.pos.y as usize][self.pos.x as usize].color = self.color;
        self.pos.x += 1;

        return self;
    }
    pub fn print(&mut self, str: &str) -> &mut Self {
        for char in str.chars() {
            self.char_at_pos(char);
        }
        return self;
    }
    pub fn println(&mut self, str: &str) -> &mut Self {
        return self.print(format!("{}\n", str).as_str());
    }
    pub fn set_cursor(&mut self, x: u16, y: u16) -> &mut Self {
        self.pos.set(Pos::from(x, y));
        return self;
    }
    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        return self;
    }
    pub fn into_renderable(&self) -> Vec<Renderable_Colors> {
        let mut vec: Vec<Renderable_Colors> = Vec::new();
        let mut pre: Option<Color> = None;
        for line in self.buf.iter() {
            for char in line.iter() {
                if let Some(c) = pre {
                    if char.color == c {
                        vec.last_mut().unwrap().text.push(char.text);
                    } else {
                        pre = Some(char.color);
                        vec.push(Renderable_Colors {
                            text: char.text.to_string(),
                            color: char.color,
                        });
                    }
                } else {
                    pre = Some(char.color);
                    vec.push(Renderable_Colors {
                        text: char.text.to_string(),
                        color: char.color,
                    });
                }
            }
            vec.last_mut().unwrap().text.push('\n');
        }
        return vec;
    }
}

pub mod Terminal {
    pub use super::helper::{key, use_alt, use_main, set_cursor, set_cursor_style, hide_cursor, show_cursor, get_cursor};
    use super::{
        helper::{self, set_color},
        Buffer,
    };
    pub fn flush(buf: &Buffer) {
        let render = buf.into_renderable();
        let pre = get_cursor();
        hide_cursor();
        for text in render.iter() {
            set_color(text.color);
            print!("{}", text.text);
        }
        set_cursor(pre);
        show_cursor();
        helper::flush();
    }
}
