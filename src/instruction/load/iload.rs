use util::code_reader::CodeReader;
use rtda::frame::Frame;
use instruction::instruction::ExecuteResult;

fn _iload(frame: Frame, index: usize) -> Frame {
    let Frame { operand_stack, local_vars } = frame;
    let val = local_vars.get_int(index);
    let operand_stack = operand_stack.push_int(val);
    Frame { operand_stack, local_vars }
}


#[allow(non_snake_case)]
fn ILOAD_0(reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    let frame = _iload(frame, 0);
    let execute_result = ExecuteResult {
        frame,
        offset: 0,
    };
    (execute_result, reader)
}


#[allow(non_snake_case)]
fn ILOAD_1(reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    let frame = _iload(frame, 1);
    let execute_result = ExecuteResult {
        frame,
        offset: 0,
    };
    (execute_result, reader)
}

#[allow(non_snake_case)]
fn ILOAD_2(reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    let frame = _iload(frame, 2);
    let execute_result = ExecuteResult {
        frame,
        offset: 0,
    };
    (execute_result, reader)
}