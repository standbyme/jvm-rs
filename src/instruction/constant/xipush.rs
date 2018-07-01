use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn BIPUSH(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("BIPUSH");

    let Frame { operand_stack, local_vars } = frame;

    let (val, code_reader) = code_reader.read_i8();
    let i = val;
    let operand_stack = operand_stack.push_int(i as i32);

    let frame = Frame { operand_stack, local_vars };
    let execute_result = ExecuteResult {
        frame,
        offset: 0,
    };
    (execute_result, code_reader)
}