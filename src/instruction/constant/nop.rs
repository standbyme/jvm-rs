use instruction::instruction::Instruction;
use util::code_reader::CodeReader;
use rtda::frame::Frame;

pub struct NOP {}

impl Instruction for NOP {
    fn execute(&self, frame: Frame) -> Frame {
        frame
    }
}

impl NOP {
    pub fn new(_reader: CodeReader) -> Box<dyn Instruction> {
        Box::new(NOP {})
    }
}