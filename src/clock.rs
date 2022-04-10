use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn init_clock() -> (Receiver<u8>, thread::JoinHandle<()>) {
    let (tx, rx): (Sender<u8>, Receiver<u8>) = mpsc::channel();
    let clock_speed = Duration::from_millis(16);

    let clock_thread = thread::spawn(move || {
        loop {
            thread::sleep(clock_speed);

            tx.send(1).unwrap();
        }
    });

    (rx, clock_thread)
}
