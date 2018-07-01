use util::code_reader::CodeReader;
use rtda::frame::Frame;
use instruction::constant::nop::NOP;

pub struct ExecuteResult {
    pub frame: Frame,
    pub offset: isize,
}


pub fn execute(opcode: u8, reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    match opcode {
        0x00 => NOP(reader, frame),

        _ => panic!("Unsupported opcode")
    }
}
