use util::code_reader::CodeReader;
use rtda::frame::Frame;
use instruction::instruction::ExecuteResult;

#[allow(non_snake_case)]
fn IADD(reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    let Frame { operand_stack, local_vars } = frame;

    let (v2, operand_stack) = operand_stack.pop_int();
    let (v1, operand_stack) = operand_stack.pop_int();
    let result = v1 + v2;
    let operand_stack = operand_stack.push_int(result);

    let frame = Frame { operand_stack, local_vars };
    let execute_result = ExecuteResult {
        frame,
        offset: 0,
    };
    (execute_result, reader)
}