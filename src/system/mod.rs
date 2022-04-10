extern crate rand;
mod display;
mod instructions;
mod sprites;
mod keyboard;

use std::fs::File;
use std::io::Read;
use std::fmt;
use display::Display;
use keyboard::Keyboard;

pub struct System {
    memory: Vec<u8>,
    program_counter: u16,
    registers: Vec<u8>,
    index_register: u16,
    stack: Vec<u16>,
    stack_pointer: u8,
    sound_timer: u8,
    delay_timer: u8,
    display: Display,
    keyboard: Keyboard,
}

impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Register:\n{:?}\nmemory\n{:?}", self.registers, self.memory)
    }
}

impl System {
    pub fn new() -> Self {
        let mut system = System {
            //memory: Vec::with_capacity(4096),
            memory: vec![0u8; 4096],
            program_counter: 0x200,
            //registers: Vec::with_capacity(16),
            registers: vec![0; 16],
            index_register: 0,
            stack: Vec::with_capacity(16),
            stack_pointer: 0,
            sound_timer: 0,
            delay_timer: 0,
            display: Display::new(),
            keyboard: Keyboard::new(),
        };

        // system.memory.extend_from_slice(&sprites::FONTS);
        system.memory.splice(0..80, sprites::FONTS);


        return system;
    }
    // TODO: update to store memory from 0x200 instead of 0
    pub fn load(&mut self, game_path: &str) {
        let file = File::open(game_path).unwrap();

        let mut game_data = Vec::new();
        for data in file.bytes() {
            let value = data.unwrap();
            game_data.push(value);
        }
        let length = 512 + game_data.len();
        self.memory[0x200..length].copy_from_slice(&game_data);

        self.display.clear();
    }
    fn fetch(&self) -> u16 {
        let pc = self.program_counter as usize;
        let mut opcode = self.memory[pc] as u16;

        opcode = opcode << 8 | self.memory[pc + 1] as u16;

        opcode
    }

    fn decode(&self, opcode: u16) -> (instructions::Instruction, Vec<u16>) {
        instructions::dissassemble(opcode)
    }

    fn increment_program_counter(&mut self) {
        self.program_counter += 2;
    }

    fn execute(&mut self, instruction: instructions::Instruction, arguments: Vec<u16>) -> bool {

        let input = self.keyboard.get_key();

        if let Some(quit) = input {
            if quit == 0xFF {
                println!("Pressing p quits the program");
                return true;
            }
        }

        match instruction.name {
            "CLS" => {
                self.display.clear();
                self.increment_program_counter();
            }
            "RETURN" => {
                let program_counter = self.stack.pop().unwrap();
                self.program_counter = program_counter;
                self.stack_pointer -= 1;
            }
            "JUMP" => {
                self.program_counter = arguments[0];
            }
            "CALL" => {
                self.stack_pointer += 1;
                self.stack.push(self.program_counter);
                self.program_counter = arguments[0]
            }
            "SKIP_EQ" => {
                if self.registers[arguments[0] as usize] == arguments[1] as u8 {
                    self.increment_program_counter();
                    self.increment_program_counter();
                } else {
                    self.increment_program_counter();
                }
            }
            "SKIP_NEQ" => {
                if self.registers[arguments[0] as usize] != arguments[1] as u8 {
                    self.increment_program_counter();
                    self.increment_program_counter();
                } else {
                    self.increment_program_counter();
                }

            }
            "SKIP_REG_EQ" => {
                if self.registers[arguments[0] as usize] == self.registers[arguments[1] as usize]  {
                    self.increment_program_counter();
                    self.increment_program_counter();
                } else {
                    self.increment_program_counter();
                }
            }
            "SKIP_REG_NEQ" => {
                if self.registers[arguments[0] as usize] != self.registers[arguments[1] as usize]  {
                    self.increment_program_counter();
                    self.increment_program_counter();
                } else {
                    self.increment_program_counter();
                }
            }
            "LOAD" => {
                self.registers[arguments[0] as usize] = arguments[1] as u8;
                self.increment_program_counter();
            }
            "ADD_REG_MEM" => {
                self.registers[arguments[0] as usize] += arguments[1] as u8;
                self.increment_program_counter();
            }
            "LOAD_REG_REG" => {
                self.registers[arguments[0] as usize] = self.registers[arguments[1] as usize];
                self.increment_program_counter();
            }
            "OR" => {
                self.registers[arguments[0] as usize] |= self.registers[arguments[1] as usize];
                self.increment_program_counter();
            }
            "AND" => {
                self.registers[arguments[0] as usize] &= self.registers[arguments[1] as usize];
                self.increment_program_counter();
            }
            "XOR" => {
                self.registers[arguments[0] as usize] ^= self.registers[arguments[1] as usize];
                self.increment_program_counter();
            }
            "ADD" => {
                let x = self.registers[arguments[0] as usize] as u16;
                let y = self.registers[arguments[1] as usize] as u16;

                let result = x + y;

                self.registers[arguments[0] as usize] = result as u8;

                if result > 255 {
                    self.registers[15] = 1;
                } else {
                    self.registers[15] = 0;
                }

                self.increment_program_counter();
            }
            "SUBTRACT" => {
                let x = self.registers[arguments[0] as usize];
                let y = self.registers[arguments[1] as usize];

                if y >= x {
                    self.registers[15] = 0;
                    self.registers[arguments[0] as usize] = y - x;
                } else {
                    self.registers[15] = 1;
                    self.registers[arguments[0] as usize] = x - y;
                }
                self.increment_program_counter();
            }
            "SHR" => {
                let x = self.registers[arguments[0] as usize];

                if x & 1 == 1 {
                    self.registers[15] = 1;
                } else {
                    self.registers[15] = 1;
                }
                self.registers[arguments[0] as usize] = x >> 1;
                self.increment_program_counter();

            }
            "SUBN" => {
                let x = self.registers[arguments[0] as usize];
                let y = self.registers[arguments[1] as usize];

                if y > x {
                    self.registers[15] = 1;
                    self.registers[arguments[0] as usize] = x - y;
                } else {
                    self.registers[15] = 0;
                    self.registers[arguments[0] as usize] = y - x;
                }
                self.increment_program_counter();
            }
            "SHL" => {
                let x = self.registers[arguments[0] as usize];

                if x > 127 {
                    self.registers[15] = 1;
                } else {
                    self.registers[15] = 1;
                }
                self.registers[arguments[0] as usize] = self.registers[arguments[0] as usize] << 1;
                self.increment_program_counter();
            }
            "SNE" => {
                if self.registers[arguments[0] as usize] == self.registers[arguments[1] as usize] {
                    self.increment_program_counter();
                } else {
                    self.increment_program_counter();
                    self.increment_program_counter();
                }
            }
            "LOAD_INDEX" => {
                self.index_register = arguments[0] as u16;
                self.increment_program_counter();
            }
            "JUMP_REG_ADDR" => {
                self.program_counter = self.registers[0] as u16 + arguments[0];
                self.increment_program_counter();
            }
            "RANDOM_REG" => {
                let random_number: u8 = rand::random();

                self.registers[arguments[0] as usize] = random_number & arguments[1] as u8;
                self.increment_program_counter();
            }
            "DRAW" => {
                let range = std::ops::Range { start: self.index_register as usize, end: (self.index_register + arguments[2]) as usize };
                let lines = &self.memory[range];

                let x = self.registers[arguments[0] as usize];
                let y = self.registers[arguments[1] as usize];

                let erase = self.display.draw(lines, x.into(), y.into());

                if erase {
                    self.registers[15] = 1;
                } else {
                    self.registers[15] = 0;
                }

                self.increment_program_counter();
            }
            "SKIP_KEY_EQ" => {
                if let Some(key) = input {
                    if self.registers[arguments[0] as usize] == key {
                        self.increment_program_counter();
                        self.increment_program_counter();
                    } else {
                        self.increment_program_counter();
                    }
                } else {
                    self.increment_program_counter();
                }
            }
            "SKIP_KEY_NEQ" => {
                if let Some(key) = input {
                    if self.registers[arguments[0] as usize] == key {
                        self.increment_program_counter();
                    } else {
                        self.increment_program_counter();
                        self.increment_program_counter();
                    }
                } else {
                    self.increment_program_counter();
                }
            }
            "LOAD_REG_DELAY" => {
                self.registers[arguments[0] as usize] = self.delay_timer;
                self.increment_program_counter();
            }
            "WAIT_KEY" => {
                if let Some(key) = input {
                    self.registers[arguments[0] as usize] = key;
                    self.increment_program_counter();
                }
            }
            "LOAD_DELAY_REG" => {
                self.delay_timer = self.registers[arguments[0] as usize];
                self.increment_program_counter();
            }
            "LOAD_SOUND_REG" => {
                self.sound_timer = self.registers[arguments[0] as usize];
                self.increment_program_counter();
            }
            "ADD_INDEX" => {
                self.index_register += self.registers[arguments[0] as usize] as u16;
                self.increment_program_counter();
            }
            "LOAD_SPRITE" => {
                self.index_register = self.registers[arguments[0] as usize] as u16 * 5;
                self.increment_program_counter();
            }
            "LOAD_BCD" => {
                let mut number = self.registers[arguments[0] as usize];
                let index = self.index_register as usize;

                self.memory[index] = number / 100;
                number %= 100;
                self.memory[index + 1] = number / 10;
                number %= 10;
                self.memory[index + 2] = number;

                self.increment_program_counter();
            }
            "LOAD_MEMORY" => {
                let index = self.index_register;

                for key in 0..16 {
                    let memory_index = key + index as usize;

                    self.memory[memory_index] = self.registers[key];
                }

                self.increment_program_counter();
            }
            "LOAD_REG" => {
                let index = self.index_register;

                for key in 0..16 {
                    let memory_index = key + index as usize;

                    self.registers[key] = self.memory[memory_index];
                }

                self.increment_program_counter();
            }
            _ => {
                println!("\n{:?} fail", instruction)
            }
        }

        self.display.print_registers(&self.registers);

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        return false;
    }

    pub fn step(&mut self) -> bool {
        let opcode = self.fetch();
        let ( instruction, arguments ) = self.decode(opcode);

        // println!("{:?}{:?}", instruction, arguments);
        self.execute(instruction, arguments)
    }
}

