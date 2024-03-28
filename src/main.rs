pub mod terminal;
use std::vec;

use terminal::{
    clear, disable, enable, key, reset_color, set_background, set_color, set_cursor,
    set_cursor_style, Color, CursorStyle, KeyCode, KeyEventKind, Pos,
};

fn main() {
    reset_color();
    clear();
    enable();
    let mut exit = false;
    let mut mode = Modes::Normal;
    let mut pos = Pos::from(0, 0);
    let mut text: Vec<Vec<char>> = vec![Vec::new()];
    loop {
        if exit {
            break;
        }
        update(&mut exit, &mut mode, &mut pos, &mut text);
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

fn update(exit: &mut bool, mode: &mut Modes, pos: &mut Pos, text: &mut Vec<Vec<char>>) {
    show_text(text);
    set_cursor(pos.to_pos());
    match *mode {
        Modes::Normal => normal(exit, mode, pos, text),
        Modes::Insert => insert(mode, pos, text),
        Modes::Replace => replace(mode, pos, text),
    }
}

fn normal(exit: &mut bool, mode: &mut Modes, pos: &mut Pos, text: &mut Vec<Vec<char>>) {
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
        }
    }
}

fn insert(mode: &mut Modes, pos: &mut Pos, text: &mut Vec<Vec<char>>) {
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
                            if (pos.y != 0) {
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
        }
    }
}

fn replace(mode: &mut Modes, pos: &mut Pos, text: &mut Vec<Vec<char>>) {
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
                    let mut a: Option<Vec<char>> = None;
                    if let Some(l) = text.get_mut(usize::from(pos.y)) {
                        if pos.x == 0 {
                            if (pos.y != 0) {
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
        }
    }
}

fn show_text(text: &Vec<Vec<char>>) {
    for (y, line) in text.iter().enumerate() {
        set_cursor(Pos::from(0, y.try_into().expect("conversion failed")));
        print!("{}", String::from_iter(line));
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
