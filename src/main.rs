// Resolution: 64 x 32
mod system;
mod clock;
use system::System;
use clock::init_clock;
use std::env;

fn main() {
    let mut rom = "./game/chip8";

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        rom = &args[1];
    }

    let mut game = System::new();
    game.load(rom);

    let (rx, _thread) = init_clock();

    loop {
        rx.recv().unwrap();
        if game.step() {
            break
        }
    }
}
