use util::code_reader::CodeReader;
use instruction::constant::nop::NOP;
use rtda::frame::Frame;

pub trait Instruction {
    fn execute(&self, frame: Frame) -> Frame;
}

impl dyn Instruction {
    pub fn new(opcode: u8, reader: CodeReader) -> Box<dyn Instruction> {
        match opcode {
            0x00 => NOP::new(reader),

            _ => panic!("Unsupported opcode")
        }
    }
}