use instruction::instruction::Instruction;
use util::code_reader::CodeReader;
use rtda::frame::Frame;

fn _istore(frame: Frame, index: usize) -> Frame {
    let Frame { operand_stack, local_vars } = frame;
    let (val, operand_stack) = operand_stack.pop_int();
    let local_vars = local_vars.set_int(index, val);
    Frame { operand_stack, local_vars }
}

#[allow(non_camel_case_types)]
pub struct ISTORE_1 {}

impl Instruction for ISTORE_1 {
    fn execute(&self, frame: Frame) -> Frame {
        _istore(frame, 1)
    }
}

impl ISTORE_1 {
    pub fn new(reader: CodeReader) -> (Box<dyn Instruction>, CodeReader) {
        (Box::new(ISTORE_1 {}), reader)
    }
}

#[allow(non_camel_case_types)]
pub struct ISTORE_2 {}

impl Instruction for ISTORE_2 {
    fn execute(&self, frame: Frame) -> Frame {
        _istore(frame, 2)
    }
}

impl ISTORE_2 {
    pub fn new(reader: CodeReader) -> (Box<dyn Instruction>, CodeReader) {
        (Box::new(ISTORE_2 {}), reader)
    }
}