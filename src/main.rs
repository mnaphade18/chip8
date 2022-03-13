// Resolution: 64 x 32
mod system;
use system::System;

fn main() {
    let mut game = System::new();

    game.load("./game/chip8");

    let mut i = 0;

    println!("Starting game");
    loop {
        //if i > 6 { break; }
        game.step();
        i += 1;
    }

    // println!("Hello, world!{}", game);
}
