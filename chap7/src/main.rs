mod movement;

use movement::vertical;
use movement::horizontal::{horizontal, left, right};

fn main() {
    println!("Hello, world!");
    vertical();
    horizontal();
    left();
    right();
}
