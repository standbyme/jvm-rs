use util::code_reader::CodeReader;
use rtda::frame::Frame;
use instruction::instruction::ExecuteResult;

#[allow(non_snake_case)]
fn _icmpPop(frame: Frame) -> (i32, i32, Frame) {
    let Frame { operand_stack, local_vars } = frame;

    let (val2, operand_stack) = operand_stack.pop_int();
    let (val1, operand_stack) = operand_stack.pop_int();

    let frame = Frame { operand_stack, local_vars };
    (val1, val2, frame)
}

#[allow(non_snake_case)]
pub fn IF_ICMPGT(reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    let (offset, reader) = reader.read_i16();

    let (val1, val2, frame) = _icmpPop(frame);
    let offset = if val1 > val2 {
        offset as isize
    } else {
        0
    };

    let execute_result = ExecuteResult {
        frame,
        offset,
    };
    (execute_result, reader)
}