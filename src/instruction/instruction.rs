use instruction::comparison::if_icmp::*;
use instruction::constant::nop::NOP;
use instruction::constant::xconst::*;
use instruction::constant::xipush::*;
use instruction::control::goto::*;
use instruction::load::iload::*;
use instruction::math::add::*;
use instruction::math::inc::*;
use instruction::store::istore::*;
use rtda::frame::Frame;
use util::code_reader::CodeReader;

pub struct ExecuteResult {
    pub frame: Frame,
    pub offset: isize,
}

pub fn execute(opcode: u8, code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    let instruction = match opcode {
        0x00 => NOP,
        0x03 => ICONST_0,
        0x04 => ICONST_1,
        0x06 => ICONST_3,
        0x3c => ISTORE_1,
        0x3d => ISTORE_2,
        0x10 => BIPUSH,
        0x1b => ILOAD_1,
        0x1c => ILOAD_2,
        0xa3 => IF_ICMPGT,
        0x60 => IADD,
        0x84 => IINC,
        0xa7 => GOTO,
        _ => {
            println!("{:?}", frame);
            panic!("Unsupported opcode")
        }
    };

    instruction(code_reader, frame)
}
