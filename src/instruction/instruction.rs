use instruction::comparison::dcmp::*;
use instruction::comparison::fcmp::*;
use instruction::comparison::if_icmp::*;
use instruction::comparison::ifcond::*;
use instruction::comparison::lcmp::*;
use instruction::constant::nop::NOP;
use instruction::constant::xconst::*;
use instruction::constant::xipush::*;
use instruction::control::goto::*;
use instruction::load::iload::*;
use instruction::math::add::*;
use instruction::math::and::*;
use instruction::math::inc::*;
use instruction::math::mul::*;
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
        0x02 => ICONST_M1,
        0x03 => ICONST_0,
        0x04 => ICONST_1,
        0x05 => ICONST_2,
        0x06 => ICONST_3,
        0x07 => ICONST_4,
        0x08 => ICONST_5,
        0x09 => LCONST_0,
        0x0A => LCONST_1,
        0x0B => FCONST_0,
        0x0C => FCONST_1,
        0x0D => FCONST_2,
        0x0E => DCONST_0,
        0x0F => DCONST_1,
        0x10 => BIPUSH,
        0x1B => ILOAD_1,
        0x1C => ILOAD_2,
        0x3C => ISTORE_1,
        0x3D => ISTORE_2,
        0x60 => IADD,
        0x68 => IMUL,
        0x69 => LMUL,
        0x6A => FMUL,
        0x6B => DMUL,
        0x7E => IAND,
        0x7F => LAND,
        0x84 => IINC,
        0x94 => LCMP,
        0x95 => FCMPL,
        0x96 => FCMPG,
        0x97 => DCMPL,
        0x98 => DCMPG,
        0x99 => IFEQ,
        0x9A => IFNE,
        0x9B => IFLT,
        0x9C => IFGE,
        0x9D => IFGT,
        0x9E => IFLE,
        0x9F => IF_ICMPEQ,
        0xA0 => IF_ICMPNE,
        0xA1 => IF_ICMPLT,
        0xA2 => IF_ICMPGE,
        0xA3 => IF_ICMPGT,
        0xA4 => IF_ICMPLE,
        0xA7 => GOTO,
        _ => {
            println!("{:?}", frame);
            panic!("Unsupported opcode : {:X}", opcode)
        }
    };

    instruction(code_reader, frame)
}
