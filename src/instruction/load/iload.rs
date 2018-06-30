use instruction::instruction::Instruction;
use util::code_reader::CodeReader;
use rtda::frame::Frame;

fn _iload(frame: Frame, index: usize) -> Frame {
    let Frame { operand_stack, local_vars } = frame;
    let val = local_vars.get_int(index);
    let operand_stack = operand_stack.push_int(val);
    Frame { operand_stack, local_vars }
}

#[allow(non_camel_case_types)]
pub struct ILOAD_0 {}

impl Instruction for ILOAD_0 {
    fn execute(&self, frame: Frame) -> Frame {
        _iload(frame, 0)
    }
}

impl ILOAD_0 {
    pub fn new(reader: CodeReader) -> (Box<dyn Instruction>, CodeReader) {
        (Box::new(ILOAD_0 {}), reader)
    }
}

#[allow(non_camel_case_types)]
pub struct ILOAD_1 {}

impl Instruction for ILOAD_1 {
    fn execute(&self, frame: Frame) -> Frame {
        _iload(frame, 1)
    }
}

impl ILOAD_1 {
    pub fn new(reader: CodeReader) -> (Box<dyn Instruction>, CodeReader) {
        (Box::new(ILOAD_1 {}), reader)
    }
}

#[allow(non_camel_case_types)]
pub struct ILOAD_2 {}

impl Instruction for ILOAD_2 {
    fn execute(&self, frame: Frame) -> Frame {
        _iload(frame, 2)
    }
}

impl ILOAD_2 {
    pub fn new(reader: CodeReader) -> (Box<dyn Instruction>, CodeReader) {
        (Box::new(ILOAD_2 {}), reader)
    }
}