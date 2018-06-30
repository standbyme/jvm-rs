extern crate byteorder;

use self::byteorder::{ByteOrder, BigEndian};
use std;

pub struct CodeReader<'a> {
    code: &'a Vec<u8>,
    pc: usize,
}

impl<'a> CodeReader<'a> {
    pub fn new(code: &Vec<u8>) -> CodeReader {
        CodeReader {
            code,
            pc: 0,
        }
    }

    pub fn set_pc(&mut self, pc: usize) {
        self.pc = pc;
    }

    pub fn read_u8(&mut self) -> u8 {
        let pc = self.pc;
        let v = self.code[pc];
        self.pc = pc + 1;
        v
    }

    pub fn read_i8(&mut self) -> i8 {
        let pc = self.pc;
        let v = self.code[pc];
        self.pc = pc + 1;
        unsafe { std::mem::transmute::<u8, i8>(v) }
    }

    pub fn read_u16(&mut self) -> u16 {
        let pc = self.pc;
        let seq = &self.code[pc..(pc + 1)];
        self.pc = pc + 2;
        BigEndian::read_u16(&seq)
    }

    pub fn read_i16(&mut self) -> i16 {
        let pc = self.pc;
        let seq = &self.code[pc..(pc + 1)];
        self.pc = pc + 2;
        BigEndian::read_i16(&seq)
    }
}