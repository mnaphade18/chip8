use std::fs::File;
use std::io::Read;
mod instructions;

#[derive(Debug)]
pub struct System {
    memory: Vec<u8>,
    program_counter: u16,
    registers: Vec<u8>,
    index_register: u8,
    stack: Vec<u16>,
    stack_pointer: u8,
    sound_timer: u8,
    delay_timer: u8,
}


impl System {
    pub fn new() -> Self {
        System {
            memory: Vec::with_capacity(4096),
            program_counter: 0x00,
            //registers: Vec::with_capacity(16),
            registers: (0..16).collect(),
            index_register: 0,
            stack: Vec::with_capacity(16),
            stack_pointer: 0,
            sound_timer: 0,
            delay_timer: 0,
        }
    }
    // TODO: update to store memory from 0x200 instead of 0
    pub fn load(&mut self, game_path: &str) {
        let file = File::open(game_path).unwrap();

        for data in file.bytes() {
            let value = data.unwrap();

            self.memory.push(value);
        }
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

    fn execute(&mut self, instruction: instructions::Instruction, arguments: Vec<u16>) {
        match instruction.name {
            "ADD" => {
                let register_1 = self.registers.get(arguments[1] as usize).unwrap().clone();
                let mut register_0 = self.registers.get_mut(arguments[0] as usize).unwrap();

                *register_0 = *register_0 + register_1;

                println!("{:?}", self.registers);

            }
            _ => {
                println!("\n{:?} fail", instruction)
            }
        }
    }

    pub fn step(&mut self) {
        let opcode = self.fetch();
        let ( instruction, arguments ) = self.decode(opcode);

        self.execute(instruction, arguments);
    }
}

