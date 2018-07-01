use util::code_reader::CodeReader;
use rtda::frame::Frame;
use instruction::instruction::ExecuteResult;

fn _istore(frame: Frame, index: usize) -> Frame {
    let Frame { operand_stack, local_vars } = frame;
    let (val, operand_stack) = operand_stack.pop_int();
    let local_vars = local_vars.set_int(index, val);
    Frame { operand_stack, local_vars }
}


#[allow(non_snake_case)]
pub fn ISTORE_1(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("ISTORE_1");

    let frame = _istore(frame, 1);

    let execute_result = ExecuteResult {
        frame,
        offset: 0,
    };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn ISTORE_2(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("ISTORE_2");

    let frame = _istore(frame, 2);

    let execute_result = ExecuteResult {
        frame,
        offset: 0,
    };
    (execute_result, code_reader)
}