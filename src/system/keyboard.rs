use std::io::Read;
use termion::{ AsyncReader, async_stdin };

pub struct Keyboard {
    input: std::io::Bytes<AsyncReader>
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            input: async_stdin().bytes(),
        }
    }

    pub fn get_key(&mut self) -> Option<u8> {
        let b = self.input.next();
        match b {
            Some(Ok(b'1')) => Some(0x1),
            Some(Ok(b'2')) => Some(0x2),
            Some(Ok(b'3')) => Some(0x3),
            Some(Ok(b'4')) => Some(0xC),
            Some(Ok(b'q')) => Some(0x4),
            Some(Ok(b'w')) => Some(0x5),
            Some(Ok(b'e')) => Some(0x6),
            Some(Ok(b'r')) => Some(0xD),
            Some(Ok(b'a')) => Some(0x7),
            Some(Ok(b's')) => Some(0x8),
            Some(Ok(b'd')) => Some(0x9),
            Some(Ok(b'f')) => Some(0xE),
            Some(Ok(b'z')) => Some(0xA),
            Some(Ok(b'x')) => Some(0x0),
            Some(Ok(b'c')) => Some(0xB),
            Some(Ok(b'v')) => Some(0xF),
            Some(Ok(b'p')) => Some(0xFF),
            _ => None
        }
    }
}
