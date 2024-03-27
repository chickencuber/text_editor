pub mod terminal;
use terminal::{
    Color,
    set_color,
    reset_color,
    set_background,
    clear,
    get_cursor,
};

fn main() {
    clear();
    set_color(Color::Red);
    println!("Hello, Red!");
    set_color(Color::Blue);
    println!("Hello, Blue!");
   set_background(Color::White);
   set_color(Color::Black);
   println!("Hello");
   reset_color();
   let pos = get_cursor();
   println!("x: {}, y: {}, ", pos.x, pos.y);
}
