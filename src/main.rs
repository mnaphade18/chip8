// Resolution: 64 x 32
mod system;

use system::System;

fn main() {
    let mut game = System::new();

    game.load("./game/data");

    game.step();

    println!("Hello, world!{:?}", game);
}
