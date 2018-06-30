use instruction::instruction::Instruction;
use util::code_reader::CodeReader;
use rtda::frame::Frame;

#[allow(non_camel_case_types)]
pub struct DCONST_0 {}

impl Instruction for DCONST_0 {
    fn execute(&self, frame: Frame) -> Frame {
        let operand_stack = frame.operand_stack.push_double(0f64);
        let local_vars = frame.local_vars;
        Frame { local_vars, operand_stack }
    }
}

impl DCONST_0 {
    pub fn new(_reader: CodeReader) -> Box<dyn Instruction> {
        Box::new(DCONST_0 {})
    }
}

#[allow(non_camel_case_types)]
pub struct ICONST_0 {}

impl Instruction for ICONST_0 {
    fn execute(&self, frame: Frame) -> Frame {
        let operand_stack = frame.operand_stack.push_int(0);
        let local_vars = frame.local_vars;
        Frame { local_vars, operand_stack }
    }
}

impl ICONST_0 {
    pub fn new(_reader: CodeReader) -> Box<dyn Instruction> {
        Box::new(ICONST_0 {})
    }
}

#[allow(non_camel_case_types)]
pub struct ICONST_1 {}

impl Instruction for ICONST_1 {
    fn execute(&self, frame: Frame) -> Frame {
        let operand_stack = frame.operand_stack.push_int(1);
        let local_vars = frame.local_vars;
        Frame { local_vars, operand_stack }
    }
}

impl ICONST_1 {
    pub fn new(_reader: CodeReader) -> Box<dyn Instruction> {
        Box::new(ICONST_1 {})
    }
}