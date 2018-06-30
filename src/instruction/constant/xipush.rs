use instruction::instruction::Instruction;
use util::code_reader::CodeReader;
use rtda::frame::Frame;

#[allow(non_camel_case_types)]
pub struct BIPUSH {
    val: i8
}

impl Instruction for BIPUSH {
    fn execute(&self, frame: Frame) -> Frame {
        let i = self.val;
        let Frame { operand_stack, local_vars } = frame;
        let operand_stack = operand_stack.push_int(i as i32);
        Frame { local_vars, operand_stack }
    }
}

impl BIPUSH {
    pub fn new(reader: CodeReader) -> (Box<dyn Instruction>, CodeReader) {
        let (val, code_reader) = reader.read_i8();
        (Box::new(BIPUSH { val }), code_reader)
    }
}