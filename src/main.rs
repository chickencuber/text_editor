pub mod terminal;
use std::vec;

use terminal::{
    clear, disable, enable, key, reset_color, set_color, set_cursor, set_cursor_style, Color,
    CursorStyle, KeyCode, KeyEventKind, Pos,
};

use std::fs;

fn get_file(location: &str) -> String {
    match fs::read_to_string(location) {
        Ok(contents) => {
            return contents
        }
        Err(_) => {
            return String::from("");
        }
    }
}

fn main() {
    reset_color();
    clear();
    enable();
    let mut exit = false;
    let mut mode = Modes::Normal;
    let mut pos = Pos::from(0, 0);
    let mut text: Vec<Vec<char>> = vec![Vec::new()];
    let plugin = Plugin::from("");
    show_text(&text, &plugin);
    loop {
        if exit {
            break;
        }
        update(&mut exit, &mut mode, &mut pos, &mut text, &plugin);
    }
    disable();
    reset_color();
    clear();
    set_cursor_style(CursorStyle::DefaultUserShape);
}

enum Modes {
    Normal,
    Insert,
    Replace,
}

fn update(
    exit: &mut bool,
    mode: &mut Modes,
    pos: &mut Pos,
    text: &mut Vec<Vec<char>>,
    plugin: &Plugin,
) {
    set_cursor(pos.to_pos());
    match *mode {
        Modes::Normal => normal(exit, mode, pos, text, plugin),
        Modes::Insert => insert(mode, pos, text, plugin),
        Modes::Replace => replace(mode, pos, text, plugin),
    }
    set_cursor(pos.to_pos());
}

fn normal(
    exit: &mut bool,
    mode: &mut Modes,
    pos: &mut Pos,
    text: &mut Vec<Vec<char>>,
    plugin: &Plugin,
) {
    set_cursor_style(CursorStyle::BlinkingBlock);
    if let Some(k) = key() {
        if let Some(key) = k.filter(KeyEventKind::Release) {
            match key.key {
                KeyCode::Char('q') => *exit = true,
                KeyCode::Char('i') => *mode = Modes::Insert,
                KeyCode::Insert => *mode = Modes::Insert,
                KeyCode::Left => {
                    if pos.x != 0 {
                        pos.x -= 1;
                    }
                }
                KeyCode::Right => {
                    if pos.x < x_size(text, pos.y) {
                        pos.x += 1;
                    }
                }
                KeyCode::Up => {
                    if pos.y != 0 {
                        pos.y -= 1;
                    }
                }
                KeyCode::Down => {
                    if pos.y < (text.len() - 1).try_into().expect("conversion failed") {
                        pos.y += 1;
                    }
                }
                KeyCode::Home => pos.x = 0,
                KeyCode::End => pos.x = x_size(text, pos.y),
                _ => {}
            }
            show_text(text, plugin);
        }
    }
}

fn insert(mode: &mut Modes, pos: &mut Pos, text: &mut Vec<Vec<char>>, plugin: &Plugin) {
    set_cursor_style(CursorStyle::BlinkingBar);
    if let Some(k) = key() {
        if let Some(key) = k.filter(KeyEventKind::Release) {
            match key.key {
                KeyCode::Insert => *mode = Modes::Replace,
                KeyCode::Esc => *mode = Modes::Normal,
                KeyCode::Char(c) => {
                    let l = text.get_mut(usize::from(pos.y));
                    if let Some(line) = l {
                        line.insert(usize::from(pos.x), c);
                        pos.x += 1;
                    }
                }
                KeyCode::Enter => {
                    if let Some(l) = text.get_mut(usize::from(pos.y)) {
                        let h = l.drain(0..usize::from(pos.x)).collect();
                        let len = l.len();
                        set_cursor(Pos::from(0, pos.y));
                        print!("{}", "     ".repeat(len));
                        text.insert(usize::from(pos.y), h);
                        pos.y += 1;
                        pos.x = 0;
                    }
                }
                KeyCode::Backspace => {
                    let mut a: Option<Vec<char>> = None;
                    if let Some(l) = text.get_mut(usize::from(pos.y)) {
                        if pos.x == 0 {
                            if pos.y != 0 {
                                let len = l.len();
                                a = Some(l.clone());
                                text.drain(usize::from(pos.y)..usize::from(pos.y + 1));
                                set_cursor(Pos::from(0, pos.y));
                                print!("{}", "     ".repeat(len));
                                set_cursor(Pos::from(
                                    0,
                                    (text.len()).try_into().expect("conversion failed"),
                                ));
                                print!(
                                    "{}",
                                    "     ".repeat(usize::from(x_size(
                                        text,
                                        (text.len() - 1).try_into().expect("conversion failed")
                                    )))
                                );
                                pos.y -= 1;
                                pos.x = x_size(text, pos.y);
                            }
                        } else {
                            let len = l.len();
                            l.drain(usize::from(pos.x - 1)..usize::from(pos.x));
                            set_cursor(Pos::from(0, pos.y));
                            print!("{}", "     ".repeat(len));
                            pos.x -= 1;
                        }
                    }
                    if let Some(add) = a {
                        if let Some(pl) = text.get_mut(usize::from(pos.y)) {
                            pl.extend(add.iter())
                        }
                    }
                }
                KeyCode::Tab => {
                    let l = text.get_mut(usize::from(pos.y));
                    if let Some(line) = l {
                        line.insert(usize::from(pos.x), ' ');
                        pos.x += 1;
                        line.insert(usize::from(pos.x), ' ');
                        pos.x += 1;
                    }
                }
                KeyCode::Left => {
                    if pos.x != 0 {
                        pos.x -= 1;
                    }
                }
                KeyCode::Right => {
                    if pos.x < x_size(text, pos.y) {
                        pos.x += 1;
                    }
                }
                KeyCode::Up => {
                    if pos.y != 0 {
                        pos.y -= 1;
                    }
                }
                KeyCode::Down => {
                    if pos.y < (text.len() - 1).try_into().expect("conversion failed") {
                        pos.y += 1;
                    }
                }
                KeyCode::Home => pos.x = 0,
                KeyCode::End => pos.x = x_size(text, pos.y),
                _ => {}
            }
            show_text(text, plugin);
        }
    }
}

fn replace(mode: &mut Modes, pos: &mut Pos, text: &mut Vec<Vec<char>>, plugin: &Plugin) {
    set_cursor_style(CursorStyle::BlinkingUnderScore);
    if let Some(k) = key() {
        if let Some(key) = k.filter(KeyEventKind::Release) {
            match key.key {
                KeyCode::Insert => *mode = Modes::Insert,
                KeyCode::Esc => *mode = Modes::Normal,
                KeyCode::Char(c) => {
                    let l = text.get_mut(usize::from(pos.y));
                    if let Some(line) = l {
                        line.insert(usize::from(pos.x), c);
                        if line.len() != usize::from(pos.x + 1) {
                            line.remove(usize::from(pos.x + 1));
                        }
                        pos.x += 1;
                    }
                }
                KeyCode::Enter => {
                    if let Some(l) = text.get_mut(usize::from(pos.y)) {
                        let h = l.drain(0..usize::from(pos.x)).collect();
                        let len = l.len();
                        set_cursor(Pos::from(0, pos.y));
                        print!("{}", "     ".repeat(len));
                        text.insert(usize::from(pos.y), h);
                        pos.y += 1;
                        pos.x = 0;
                    }
                }
                KeyCode::Backspace => {
                    if pos.x == 0 {
                        if pos.y != 0 {
                            pos.y -= 1;
                            pos.x = x_size(text, pos.y);
                        }
                    } else {
                        pos.x -= 1;
                    }
                }
                KeyCode::Tab => {
                    let l = text.get_mut(usize::from(pos.y));
                    if let Some(line) = l {
                        line.insert(usize::from(pos.x), ' ');
                        if line.len() != usize::from(pos.x + 1) {
                            line.remove(usize::from(pos.x + 1));
                        }
                        pos.x += 1;
                        line.insert(usize::from(pos.x), ' ');
                        if line.len() != usize::from(pos.x + 1) {
                            line.remove(usize::from(pos.x + 1));
                        }
                        pos.x += 1;
                    }
                }
                KeyCode::Left => {
                    if pos.x != 0 {
                        pos.x -= 1;
                    }
                }
                KeyCode::Right => {
                    if pos.x < x_size(text, pos.y) {
                        pos.x += 1;
                    }
                }
                KeyCode::Up => {
                    if pos.y != 0 {
                        pos.y -= 1;
                    }
                }
                KeyCode::Down => {
                    if pos.y < (text.len() - 1).try_into().expect("conversion failed") {
                        pos.y += 1;
                    }
                }
                KeyCode::Home => pos.x = 0,
                KeyCode::End => pos.x = x_size(text, pos.y),
                _ => {}
            }
            show_text(text, plugin);
        }
    }
}

fn show_text(text: &Vec<Vec<char>>, plugin: &Plugin) {
    let flat = flatten(text);
    let tokens = plugin.tokenize(&flat);
    if tokens.len() == flat.len() {
        for (y, line) in text.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                let pos = &Pos::from(
                    x.try_into().expect("conversion failed"),
                    y.try_into().expect("conversion failed"),
                );
                set_cursor(pos.to_pos());
                if let Some(token) = tokens.get(flat_idx(text, pos.to_pos())) {
                    set_color(token.to_color());
                }
                print!("{}", c);
            }
        }
    } else {
        set_color(plugin.get_default().to_color());
        for (y, line) in text.iter().enumerate() {
            set_cursor(Pos::from(0, y.try_into().expect("conversion failed")));
            print!("{}", String::from_iter(line));
        }
    }
}

fn x_size(text: &Vec<Vec<char>>, y: u16) -> u16 {
    let l = text.get(usize::from(y));
    if let Some(line) = l {
        return (line.len()).try_into().expect("conversion failed");
    } else {
        return 0;
    }
}

fn flat_idx(text: &Vec<Vec<char>>, pos: Pos) -> usize {
    let mut idx = 0;
    for y in 0..usize::from(pos.y) {
        if let Some(line) = text.get(y) {
            idx += line.len() + 1;
        }
    }
    idx += usize::from(pos.x);
    return idx;
}

fn flatten(text: &Vec<Vec<char>>) -> Vec<char> {
    return text.clone().join(&'\n');
}

//plugin types
use mlua::prelude::*;
struct Token {
    color: String,
}

impl Token {
    pub fn to_color(&self) -> Color {
        match self.color.to_ascii_lowercase().as_str() {
            "darkgrey" => Color::DarkGrey,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "white" => Color::White,
            "black" => Color::Black,
            "darkred" => Color::DarkRed,
            "darkgreen" => Color::DarkGreen,
            "darkyellow" => Color::DarkYellow,
            "darkblue" => Color::DarkBlue,
            "darkmagenta" => Color::DarkMagenta,
            "darkcyan" => Color::DarkCyan,
            "grey" => Color::Grey,
            _ => Color::White,
        }
    }
}

struct Plugin {
    lua: Lua,
}

impl Plugin {
    pub fn tokenize(&self, text: &Vec<char>) -> Vec<Token> {
        if self.lua.globals().contains_key("tokenize").unwrap() {
            let func: LuaFunction = self.lua.globals().get("tokenize").unwrap();
            let f = text.iter().map(|v| v.to_string()).collect();
            let r = func.call::<Vec<String>, Vec<String>>(f).unwrap();
            r.iter()
                .map(|v| Token {
                    color: v.to_string(),
                })
                .collect()
        } else {
            Vec::new()
        }
    }
    pub fn get_default(&self) -> Token {
        if self.lua.globals().contains_key("get_default").unwrap() {
            let func: LuaFunction = self.lua.globals().get("get_default").unwrap();
            let r = func.call::<_, String>(()).unwrap();
            Token { color: r }
        } else {
            Token {
                color: "White".to_string(),
            }
        }
    }
    pub fn from(script: &str) -> Self {
        let lua = Lua::new();
        lua.load(script).exec().unwrap();
        Self { lua: lua }
    }
}
