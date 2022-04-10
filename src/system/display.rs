extern crate termion;
#[path="../helpers/mod.rs"]
mod helpers;

use helpers::display;
use std::io::{ Write, Stdout, stdout };
use termion::{ color, clear, cursor };
use termion::raw::{ RawTerminal, IntoRawMode };

pub struct Display {
    display: RawTerminal<Stdout>,
    memory: Vec<Vec<bool>>,
}

impl Display {
    pub fn new() -> Self {
        Display {
            display: stdout().into_raw_mode().unwrap(),
            memory: vec! [vec![false; 32]; 64],
        }
    }
    pub fn clear(&mut self) {
        write!(self.display,"{}", clear::All).unwrap();
        self.memory =  vec! [vec![false; 32]; 64];
    }

    pub fn print_registers(&mut self, registers: &Vec<u8>) {
        write!(
            self.display,
            "{}Registers: {:?}{}",
           cursor::Goto(1, 37),
           registers,
           cursor::Goto(80, 40),
        ).unwrap();
    }

    pub fn draw(&mut self, lines: &[u8], x: u16, y: u16) -> bool {
        let mut erase = false;
        for (index, line) in lines.iter().enumerate() {
            let index_y = y + index as u16;
            erase |= self.draw_line(*line, x, index_y);
        }

        erase
    }
    fn set_pixel(&mut self, x: u16, y: u16) {
        write!(
            self.display,
           "{}{} {}{}",
           color::Bg(color::Red),
           cursor::Goto(x+1, y+1),
           color::Bg(color::Reset),
           cursor::Goto(80, 40)
        ).unwrap();

    }

    fn unset_pixel(&mut self, x: u16, y: u16) {
        write!(
            self.display,
            "{}{} {}{}",
            color::Bg(color::Black),
            cursor::Goto(x+1, y+1),
            color::Bg(color::Reset),
            cursor::Goto(80, 40),
        ).unwrap();
    }

    fn draw_line(&mut self, line: u8, x: u16, unwraped_y: u16) -> bool {
        let binary_string = display::as_binary(line);
        let mut erase = false;

        for (index, bit) in binary_string.chars().enumerate() {
            let unwraped_x = x + index as u16;
            let index_x = unwraped_x % 64;

            let y = unwraped_y % 32;

            let previous_state = self.memory[index_x as usize][y as usize];
            self.memory[index_x as usize][y as usize] ^= bit == '1';

            if self.memory[index_x as usize][y as usize] {
                self.set_pixel(index_x, y);
            } else {
                self.unset_pixel(index_x, y);
            }

            erase = !self.memory[index_x as usize][y as usize] && !previous_state
        }

        erase
    }
}
