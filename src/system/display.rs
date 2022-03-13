extern crate termion;
#[path="../helpers/mod.rs"]
mod helpers;

use helpers::display;
use std::io::{ Write, Stdout, stdout };
use std::{time, thread };
use termion::{ color, clear, cursor };

#[derive(Debug)]
pub struct Display {
    display: Stdout
}

impl Display {
    pub fn new() -> Self {
        Display {
            display: stdout()
        }
    }
    pub fn clear(&mut self) {
        write!(self.display,"{}", clear::All).unwrap();
    }

    pub fn draw(&mut self, lines: &[u8], x: u16, y: u16) {
        for (index, line) in lines.iter().enumerate() {
            let index_y = y + index as u16;
            self.draw_line(*line, x, index_y);
        }
        //thread::sleep(time::Duration::new(1,0));
    }
    fn set_pixel(&mut self, x: u16, y: u16) {
        write!(self.display, "{}{} {}", color::Bg(color::Red), cursor::Goto(x, y), color::Bg(color::Reset)).unwrap();

    }

    fn unset_pixel(&mut self, x: u16, y: u16) {
        write!(self.display, "{}{} {}",color::Bg(color::Black), cursor::Goto(x, y), color::Bg(color::Reset)).unwrap();
    }

    fn draw_line(&mut self, line: u8, x: u16, y: u16) {
        let binary_string = display::as_binary(line);

        for (index, bit) in binary_string.chars().enumerate() {
            let index_x = x + index as u16;

            write!(self.display, "{} Writing for post: {:?} at :{}, {}", cursor::Goto(80, 40), binary_string, index_x, y).unwrap();

            if bit == '1' {
                self.set_pixel(index_x, y);
            } else {
                self.unset_pixel(index_x, y);
            }
        }
    }
}
