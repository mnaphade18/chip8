#[derive(Debug, Copy, Clone)]
pub struct Argument {
    mask: u16,
    shift: u8,
    type_name: char,
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    pub id: &'static str,
    pub name: &'static str,
    pub mask: u16,
    pub pattern: u16,
    pub arguments: [Argument;3],
}

const NO_ARGUMENTS: [Argument; 3] = [
    Argument { mask: 0x0000, shift: 0, type_name: 'R' },
    Argument { mask: 0x0000, shift: 0, type_name: 'R' },
    Argument { mask: 0x0000, shift: 0, type_name: 'R' },
];
pub const CLEAR: Instruction = Instruction {
    id: "CLS",
    name: "CLS",
    mask: 0x00f0,
    pattern: 0x00E0,
    arguments: NO_ARGUMENTS,
};

pub const RETURN: Instruction = Instruction {
    id: "RETURN",
    name: "RETURN",
    mask: 0x00ff,
    pattern: 0x00EE,
    arguments: NO_ARGUMENTS,
};

const JUMP_ARGUMENTS: [Argument; 3] = [
    Argument { mask: 0x0fff, shift: 0, type_name: 'R' },
    Argument { mask: 0x0000, shift: 0, type_name: 'R' },
    Argument { mask: 0x0000, shift: 0, type_name: 'R' },
];
pub const JUMP: Instruction = Instruction {
    id: "JUMP_ADDR",
    name: "JUMP",
    mask: 0xf000,
    pattern: 0x1000,
    arguments: JUMP_ARGUMENTS,
};

pub const CALL: Instruction = Instruction {
    id: "CALL_ADDR",
    name: "CALL",
    mask: 0xf000,
    pattern: 0x1000,
    arguments: JUMP_ARGUMENTS,
};

const LOAD_ARGUMENTS: [Argument; 3] = [
    Argument { mask: 0x0f00, shift: 8, type_name: 'R' },
    Argument { mask: 0x00ff, shift: 0, type_name: 'R' },
    Argument { mask: 0x0000, shift: 0, type_name: 'R' },
];

pub const SKIP_EQ: Instruction = Instruction {
    id: "SKIP_EQ_VX",
    name: "SKIP_EQ",
    mask: 0xf000,
    pattern: 0x3000,
    arguments: LOAD_ARGUMENTS,
};

pub const SKIP_NEQ: Instruction = Instruction {
    id: "SKIP_NEQ_VX",
    name: "SKIP_NEQ",
    mask: 0xf000,
    pattern: 0x4000,
    arguments: LOAD_ARGUMENTS,
};

const ADD_ARGUMENTS: [Argument; 3] = [
    Argument { mask: 0x0f00, shift: 8, type_name: 'R' },
    Argument { mask: 0x00f0, shift: 4, type_name: 'R' },
    Argument { mask: 0x0000, shift: 0, type_name: 'R' },
];
pub const SKIP_REG_EQ: Instruction = Instruction { 
    id: "SKIP_REG_EQ_VX_VY",
    name: "SKIP_REG_EQ", 
    mask: 0xf00f, 
    pattern: 0x5000,
    arguments: ADD_ARGUMENTS,
};

pub const LOAD: Instruction = Instruction {
    id: "LOAD_VX",
    name: "LOAD",
    mask: 0xf000,
    pattern: 0x6000,
    arguments: LOAD_ARGUMENTS,
};

pub const ADD_REG_MEM: Instruction = Instruction { 
    id: "ADD_VX_MEM",
    name: "ADD_REG_MEM", 
    mask: 0xf000, 
    pattern: 0x7000,
    arguments: LOAD_ARGUMENTS,
};

pub const LOAD_REG_REG: Instruction = Instruction { 
    id: "LOAD_VX_VY",
    name: "LOAD_REG_REG", 
    mask: 0xf00f, 
    pattern: 0x8000,
    arguments: ADD_ARGUMENTS,
};

pub const OR: Instruction = Instruction { 
    id: "OR_VX_VY",
    name: "OR", 
    mask: 0xf00f, 
    pattern: 0x8001,
    arguments: ADD_ARGUMENTS,
};

pub const AND: Instruction = Instruction { 
    id: "AND_VX_VY",
    name: "AND", 
    mask: 0xf00f, 
    pattern: 0x8002,
    arguments: ADD_ARGUMENTS,
};

pub const XOR: Instruction = Instruction { 
    id: "XOR_VX_VY",
    name: "XOR", 
    mask: 0xf00f, 
    pattern: 0x8003,
    arguments: ADD_ARGUMENTS,
};

pub const ADD: Instruction = Instruction { 
    id: "ADD_VX_VY",
    name: "ADD", 
    mask: 0xf00f, 
    pattern: 0x8004,
    arguments: ADD_ARGUMENTS,
};
pub const SUBTRACT: Instruction = Instruction { 
    id: "SUB_VX_VY",
    name: "SUBTRACT", 
    mask: 0xf00f, 
    pattern: 0x8005,
    arguments: ADD_ARGUMENTS,
};
pub const SHR: Instruction = Instruction { 
    id: "SHR_VX",
    name: "SHR", 
    mask: 0xf00f, 
    pattern: 0x8006,
    arguments: ADD_ARGUMENTS,
};
pub const SUBN: Instruction = Instruction { 
    id: "SUBN_VX_VY",
    name: "SUBN", 
    mask: 0xf00f, 
    pattern: 0x8007,
    arguments: ADD_ARGUMENTS,
};
pub const SHL: Instruction = Instruction { 
    id: "SHL_VX",
    name: "SHL", 
    mask: 0xf00f, 
    pattern: 0x800E,
    arguments: ADD_ARGUMENTS,
};
pub const SNE: Instruction = Instruction { 
    id: "SNE_VX_VY",
    name: "SNE", 
    mask: 0xf00f, 
    pattern: 0x9000,
    arguments: ADD_ARGUMENTS,
};
pub const LOAD_INDEX: Instruction = Instruction { 
    id: "LOAD_INDEX_ADDR",
    name: "LOAD_INDEX", 
    mask: 0xf000, 
    pattern: 0xA000,
    arguments: JUMP_ARGUMENTS,
};
pub const JUMP_REG_ADDR: Instruction = Instruction { 
    id: "JUMP_REG_ADDR",
    name: "JUMP_REG_ADDR", 
    mask: 0xf000, 
    pattern: 0xB000,
    arguments: JUMP_ARGUMENTS,
};
pub const RANDOM_REG: Instruction = Instruction { 
    id: "RANDOM_VX",
    name: "RANDOM_REG", 
    mask: 0xf000, 
    pattern: 0xC000,
    arguments: JUMP_ARGUMENTS,
};

const DRAW_ARGUMENTS: [Argument; 3] = [
    Argument { mask: 0x0f00, shift: 8, type_name: 'R' },
    Argument { mask: 0x00f0, shift: 4, type_name: 'R' },
    Argument { mask: 0x000f, shift: 0, type_name: 'R' },
];

pub const DRAW: Instruction = Instruction { 
    id: "DRAW",
    name: "DRAW", 
    mask: 0xf000, 
    pattern: 0xD000,
    arguments: DRAW_ARGUMENTS,
};

const KEY_ARGUMENT: [Argument; 3] = [
    Argument { mask: 0x0f00, shift: 8, type_name: 'R' },
    Argument { mask: 0x0000, shift: 0, type_name: 'R' },
    Argument { mask: 0x0000, shift: 0, type_name: 'R' },
];

pub const SKIP_KEY: Instruction =  Instruction {
    id: "SKIP_KEY_EQ",
    name: "SKIP_KEY_EQ",
    mask: 0xf0ff,
    pattern: 0xE09E,
    arguments: KEY_ARGUMENT,
};

pub const SKIP_KEY_NEQ: Instruction =  Instruction {
    id: "SKIP_KEY_NEQ",
    name: "SKIP_KEY_NEQ",
    mask: 0xf0ff,
    pattern: 0xE0A1,
    arguments: KEY_ARGUMENT,
};

pub const LOAD_REG_DELAY: Instruction =  Instruction {
    id: "LOAD_REG_DELAY",
    name: "LOAD_REG_DELAY",
    mask: 0xf0ff,
    pattern: 0xF007,
    arguments: KEY_ARGUMENT,
};

pub const WAIT_KEY: Instruction =  Instruction {
    id: "WAIT_KEY",
    name: "WAIT_KEY",
    mask: 0xf0ff,
    pattern: 0xF00A,
    arguments: KEY_ARGUMENT,
};

pub const LOAD_DELAY_REG: Instruction =  Instruction {
    id: "LOAD_DELAY_VX",
    name: "LOAD_DELAY_REG",
    mask: 0xf0ff,
    pattern: 0xF015,
    arguments: KEY_ARGUMENT,
};

pub const LOAD_SOUND_REG: Instruction =  Instruction {
    id: "LOAD_SOUND_VX",
    name: "LOAD_SOUND_REG",
    mask: 0xf0ff,
    pattern: 0xF018,
    arguments: KEY_ARGUMENT,
};

pub const ADD_INDEX: Instruction =  Instruction {
    id: "ADD_INDEX_VX",
    name: "ADD_INDEX",
    mask: 0xf0ff,
    pattern: 0xF01E,
    arguments: KEY_ARGUMENT,
};

pub const LOAD_SPRITE: Instruction =  Instruction {
    id: "LOAD_SPRITE_VX",
    name: "LOAD_SPRITE",
    mask: 0xf0ff,
    pattern: 0xF029,
    arguments: KEY_ARGUMENT,
};

pub const LOAD_BCD: Instruction =  Instruction {
    id: "LOAD_BCD",
    name: "LOAD_BCD_VX",
    mask: 0xf0ff,
    pattern: 0xF033,
    arguments: KEY_ARGUMENT,
};

pub const LOAD_MEMORY: Instruction =  Instruction {
    id: "LOAD_MEMORY",
    name: "LOAD_MEMORY_VX",
    mask: 0xf0ff,
    pattern: 0xF055,
    arguments: KEY_ARGUMENT,
};

pub const LOAD_REG: Instruction =  Instruction {
    id: "LOAD_VX",
    name: "LOAD_REG",
    mask: 0xf0ff,
    pattern: 0xF065,
    arguments: KEY_ARGUMENT,
};

const INSTRUCTIONS: [Instruction; 34] = [
    CLEAR, RETURN, JUMP, CALL, SKIP_EQ, SKIP_NEQ, SKIP_REG_EQ, LOAD, ADD_REG_MEM, LOAD_REG_REG, OR,
    AND, XOR, ADD, SUBTRACT, SHR, SUBN, SHL, SNE, LOAD_INDEX, JUMP_REG_ADDR, RANDOM_REG, DRAW,
    SKIP_KEY, SKIP_KEY_NEQ, LOAD_REG_DELAY, WAIT_KEY, LOAD_DELAY_REG, LOAD_SOUND_REG, ADD_INDEX,
    LOAD_SPRITE, LOAD_BCD, LOAD_MEMORY, LOAD_REG
];

pub fn dissassemble(opcode: u16) -> (Instruction, Vec<u16>) {
    let instruction = INSTRUCTIONS.iter().find(|i| (opcode & i.mask) == i.pattern ).unwrap();
    let arguments: Vec<u16> = instruction.arguments.iter().map(|a| (opcode & a.mask) >> a.shift).collect();

    println!("INSTRUCITON::{:?}\n argumnets:::{:?}", instruction, arguments);
    (instruction.clone(), arguments)
}
