pub mod terminal;
use std::{env, fs, vec};

use terminal::{
    clear_line, disable, enable, flush, key, reset_color, set_color, set_cursor, set_cursor_style,
    term_size, use_alt, use_main, Color, CursorStyle, KeyCode, KeyEventKind, Pos, AUTO_FLUSH,
};

fn get_file(location: &str) -> String {
    match fs::read_to_string(location) {
        Ok(contents) => return contents,
        Err(_) => {
            return String::from("");
        }
    }
}

fn load_text(text: &mut Vec<Vec<char>>, file: &str) {
    let file_vec: Vec<char> = file.chars().collect();
    let mut y: usize = 0;
    for c in file_vec.iter() {
        if *c == '\n' || *c == '\r' {
            text.push(Vec::new());
            y += 1;
        } else {
            if let Some(l) = text.get_mut(y) {
                l.push(*c);
            }
        }
    }
}

fn main() {
    unsafe {
        AUTO_FLUSH = false;
    }
    use_alt();
    reset_color();
    enable();
    flush();
    let mut exit = false;
    let mut mode = Modes::Normal;
    let mut pos = Pos::from(0, 0);
    let mut text: Vec<Vec<char>> = vec![Vec::new()];
    let mut scroll: u16 = 0;
    let plugin = Plugin::from(&get_file("D:/programming/Rust Projects/Projects/text_editor/test plugin.lua"));

    let args: Vec<String> = env::args().collect();

    if let Some(file) = args.get(1) {
        load_text(&mut text, get_file(file.as_str()).as_str());
    }

    show_text(&text, &plugin, &scroll, &pos);
    loop {
        if exit {
            break;
        }
        update(
            &mut exit,
            &mut mode,
            &mut pos,
            &mut text,
            &plugin,
            &mut scroll,
        );
        flush();
    }
    disable();
    reset_color();
    set_cursor_style(CursorStyle::DefaultUserShape);
    use_main();
    flush();
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
    scroll: &mut u16,
) {
    set_cursor(pos.to_pos());
    match *mode {
        Modes::Normal => normal(exit, mode, pos, text, plugin, scroll),
        Modes::Insert => insert(mode, pos, text, plugin, scroll),
        Modes::Replace => replace(mode, pos, text, plugin, scroll),
    }
    set_cursor(pos.to_pos());
}

fn normal(
    exit: &mut bool,
    mode: &mut Modes,
    pos: &mut Pos,
    text: &mut Vec<Vec<char>>,
    plugin: &Plugin,
    scroll: &mut u16,
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
                    if pos.x < x_size(text, pos.y + *scroll) {
                        pos.x += 1;
                    }
                }
                KeyCode::Up => {
                    if pos.y != 0 {
                        pos.y -= 1;
                    }
                }
                KeyCode::Down => {
                    if pos.y + *scroll < (text.len() - 1).try_into().expect("conversion failed") {
                        pos.y += 1;
                    }
                }
                KeyCode::Home => pos.x = 0,
                KeyCode::End => pos.x = x_size(text, pos.y + *scroll),
                _ => {}
            }
            calculate_scroll(scroll, pos, text);
            show_text(text, plugin, scroll, pos);
        }
    }
}

fn insert(
    mode: &mut Modes,
    pos: &mut Pos,
    text: &mut Vec<Vec<char>>,
    plugin: &Plugin,
    scroll: &mut u16,
) {
    set_cursor_style(CursorStyle::BlinkingBar);
    if let Some(k) = key() {
        if let Some(key) = k.filter(KeyEventKind::Release) {
            match key.key {
                KeyCode::Insert => *mode = Modes::Replace,
                KeyCode::Esc => *mode = Modes::Normal,
                KeyCode::Char(c) => {
                    let l = text.get_mut(usize::from(pos.y + *scroll));
                    if let Some(line) = l {
                        line.insert(usize::from(pos.x), c);
                        pos.x += 1;
                    }
                }
                KeyCode::Enter => {
                    if let Some(l) = text.get_mut(usize::from(pos.y + *scroll)) {
                        let h = l.drain(0..usize::from(pos.x)).collect();
                        let len = l.len();
                        set_cursor(Pos::from(0, pos.y));
                        print!("{}", "     ".repeat(len));
                        text.insert(usize::from(pos.y + *scroll), h);
                        pos.y += 1;
                        pos.x = 0;
                    }
                }
                KeyCode::Backspace => {
                    let mut a: Option<Vec<char>> = None;
                    if let Some(l) = text.get_mut(usize::from(pos.y + *scroll)) {
                        if pos.x == 0 {
                            if pos.y + *scroll != 0 {
                                let len = l.len();
                                a = Some(l.clone());
                                text.drain(
                                    usize::from(pos.y + *scroll)..usize::from(pos.y + 1 + *scroll),
                                );
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
                                pos.x = x_size(text, pos.y + *scroll);
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
                        if let Some(pl) = text.get_mut(usize::from(pos.y + *scroll)) {
                            pl.extend(add.iter())
                        }
                    }
                }
                KeyCode::Tab => {
                    let l = text.get_mut(usize::from(pos.y + *scroll));
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
                    if pos.x < x_size(text, pos.y + *scroll) {
                        pos.x += 1;
                    }
                }
                KeyCode::Up => {
                    if pos.y != 0 {
                        pos.y -= 1;
                    }
                }
                KeyCode::Down => {
                    if pos.y + *scroll < (text.len() - 1).try_into().expect("conversion failed") {
                        pos.y += 1;
                    }
                }
                KeyCode::Home => pos.x = 0,
                KeyCode::End => pos.x = x_size(text, pos.y + *scroll),
                _ => {}
            }
            calculate_scroll(scroll, pos, text);
            show_text(text, plugin, scroll, pos);
        }
    }
}

fn replace(
    mode: &mut Modes,
    pos: &mut Pos,
    text: &mut Vec<Vec<char>>,
    plugin: &Plugin,
    scroll: &mut u16,
) {
    set_cursor_style(CursorStyle::BlinkingUnderScore);
    if let Some(k) = key() {
        if let Some(key) = k.filter(KeyEventKind::Release) {
            match key.key {
                KeyCode::Insert => *mode = Modes::Insert,
                KeyCode::Esc => *mode = Modes::Normal,
                KeyCode::Char(c) => {
                    let l = text.get_mut(usize::from(pos.y + *scroll));
                    if let Some(line) = l {
                        line.insert(usize::from(pos.x), c);
                        if line.len() != usize::from(pos.x + 1) {
                            line.remove(usize::from(pos.x + 1));
                        }
                        pos.x += 1;
                    }
                }
                KeyCode::Enter => {
                    if let Some(l) = text.get_mut(usize::from(pos.y + *scroll)) {
                        let h = l.drain(0..usize::from(pos.x)).collect();
                        let len = l.len();
                        set_cursor(Pos::from(0, pos.y));
                        print!("{}", "     ".repeat(len));
                        text.insert(usize::from(pos.y + *scroll), h);
                        pos.y += 1;
                        pos.x = 0;
                    }
                }
                KeyCode::Backspace => {
                    if pos.x == 0 {
                        if pos.y != 0 {
                            pos.y -= 1;
                            pos.x = x_size(text, pos.y + *scroll);
                        }
                    } else {
                        pos.x -= 1;
                    }
                }
                KeyCode::Tab => {
                    let l = text.get_mut(usize::from(pos.y + *scroll));
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
                    if pos.x < x_size(text, pos.y + *scroll) {
                        pos.x += 1;
                    }
                }
                KeyCode::Up => {
                    if pos.y != 0 {
                        pos.y -= 1;
                    }
                }
                KeyCode::Down => {
                    if pos.y + *scroll < (text.len() - 1).try_into().expect("conversion failed") {
                        pos.y += 1;
                    }
                }
                KeyCode::Home => pos.x = 0,
                KeyCode::End => pos.x = x_size(text, pos.y + *scroll),
                _ => {}
            }
            calculate_scroll(scroll, pos, text);
            show_text(text, plugin, scroll, pos);
        }
    }
}

fn calculate_scroll(scroll: &mut u16, pos: &mut Pos, text: &Vec<Vec<char>>) {
    if !(pos.x < x_size(text, pos.y + *scroll)) {
        pos.x = x_size(text, pos.y + *scroll);
    }
    let height = term_size().1;
    let len: u16 = text.len().try_into().expect("conversion failed");
    if len <= height {
        *scroll = 0;
        return;
    }
    if pos.y == height / 2 {
        return;
    }
    if pos.y >= height / 2 {
        if *scroll == len - height {
            return;
        }
        *scroll = (*scroll).saturating_add(pos.y - height / 2);
        pos.y = height / 2;
    } else {
        if *scroll == 0 {
            return;
        }
        *scroll = (*scroll).saturating_sub(height / 2 - pos.y);
        pos.y = height / 2;
    }
}

use cond_utils::Between;

fn show_text(text: &Vec<Vec<char>>, plugin: &Plugin, scroll: &u16, pos: &Pos) {
    let flat = flatten(text);
    let tokens = plugin.tokenize(&flat);
    let (width, height) = term_size();
    let mut x_scroll: u16 = 0;
    if pos.x > width - 1 {
        x_scroll = pos.x - width + 1;
    }
    let area = *scroll..=*scroll + height - 1;
    if tokens.len() == flat.len() {
        for (mut y, line) in text.iter().enumerate() {
            if !y.within(usize::from(*area.start()), usize::from(*area.end())) {
                if y > usize::from(*area.end()) {
                    break;
                }
                continue;
            }
            y -= usize::from(*scroll);
            clear_line(y.try_into().expect("conversion failed"));
            for (mut x, c) in line.iter().enumerate() {
                if !x.within(usize::from(x_scroll), usize::from(x_scroll + width - 1)) {
                    if x > usize::from(x_scroll + width - 1) {
                        break;
                    }
                    continue;
                }
                x -= usize::from(x_scroll);
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
        for (mut y, line) in text.iter().enumerate() {
            if !y.within(usize::from(*area.start()), usize::from(*area.end())) {
                if y > usize::from(*area.end()) {
                    break;
                }
                continue;
            }
            y -= usize::from(*scroll);
            clear_line(y.try_into().expect("conversion failed"));
            for (mut x, c) in line.iter().enumerate() {
                if !x.within(usize::from(x_scroll), usize::from(x_scroll + width - 1)) {
                    if x > usize::from(x_scroll + width - 1) {
                        break;
                    }
                    continue;
                }
                x -= usize::from(x_scroll);
                let pos = &Pos::from(
                    x.try_into().expect("conversion failed"),
                    y.try_into().expect("conversion failed"),
                );
                set_cursor(pos.to_pos());
                print!("{}", c);
            }
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
