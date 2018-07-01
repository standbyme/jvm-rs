use util::code_reader::CodeReader;
use rtda::frame::Frame;
use instruction::instruction::ExecuteResult;


#[allow(non_snake_case)]
fn DCONST_0(reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    let Frame { operand_stack, local_vars } = frame;
    let operand_stack = operand_stack.push_double(0f64);
    let local_vars = local_vars;
    let frame = Frame { operand_stack, local_vars };

    let execute_result = ExecuteResult {
        frame,
        offset: 0,
    };
    (execute_result, reader)
}

#[allow(non_snake_case)]
fn ICONST_0(reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    let Frame { operand_stack, local_vars } = frame;
    let operand_stack = operand_stack.push_int(0);
    let local_vars = local_vars;
    let frame = Frame { operand_stack, local_vars };

    let execute_result = ExecuteResult {
        frame,
        offset: 0,
    };
    (execute_result, reader)
}

#[allow(non_snake_case)]
fn ICONST_1(reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    let Frame { operand_stack, local_vars } = frame;
    let operand_stack = operand_stack.push_int(1);
    let local_vars = local_vars;
    let frame = Frame { operand_stack, local_vars };

    let execute_result = ExecuteResult {
        frame,
        offset: 0,
    };
    (execute_result, reader)
}