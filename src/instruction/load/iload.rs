use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use util::code_reader::CodeReader;

fn _iload(frame: Frame, index: usize) -> Frame {
    let Frame {
        operand_stack,
        local_vars,
    } = frame;
    let val = local_vars.get_int(index);
    let operand_stack = operand_stack.push_int(val);
    Frame {
        operand_stack,
        local_vars,
    }
}

#[allow(non_snake_case)]
pub fn ILOAD_0(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("ILOAD_0");

    let frame = _iload(frame, 0);
    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn ILOAD_1(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("ILOAD_1");

    let frame = _iload(frame, 1);
    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn ILOAD_2(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("ILOAD_2");

    let frame = _iload(frame, 2);
    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}
