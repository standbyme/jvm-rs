use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
fn BIPUSH(reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    let Frame { operand_stack, local_vars } = frame;

    let (val, reader) = reader.read_i8();
    let i = val;
    let operand_stack = operand_stack.push_int(i as i32);

    let frame = Frame { operand_stack, local_vars };
    let execute_result = ExecuteResult {
        frame,
        offset: 0,
    };
    (execute_result, reader)
}