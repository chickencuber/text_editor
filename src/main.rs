mod terminal;

use terminal::{Buffer, KeyEventKind, Terminal, KeyCode, Color};

fn main() {
    Terminal::use_alt();
    let mut buf = Buffer::new();
    buf.println("Hello World!");
    buf.set_color(Color::Yellow);
    buf.print("hi");
    Terminal::flush(&buf);
    loop {
        if let Some(key) = Terminal::key() {
            if let Some(k) = key.only(KeyEventKind::Press) {
                if k.key == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }
    Terminal::use_main();
}
