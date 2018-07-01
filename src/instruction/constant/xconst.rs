use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn DCONST_0(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("DCONST_0");
    let Frame {
        operand_stack,
        local_vars,
    } = frame;
    let operand_stack = operand_stack.push_double(0f64);
    let local_vars = local_vars;
    let frame = Frame {
        operand_stack,
        local_vars,
    };

    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn ICONST_0(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("ICONST_0");

    let Frame {
        operand_stack,
        local_vars,
    } = frame;
    let operand_stack = operand_stack.push_int(0);
    let local_vars = local_vars;
    let frame = Frame {
        operand_stack,
        local_vars,
    };

    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn ICONST_1(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("ICONST_1");

    let Frame {
        operand_stack,
        local_vars,
    } = frame;
    let operand_stack = operand_stack.push_int(1);
    let local_vars = local_vars;
    let frame = Frame {
        operand_stack,
        local_vars,
    };

    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn ICONST_3(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("ICONST_3");

    let Frame {
        operand_stack,
        local_vars,
    } = frame;
    let operand_stack = operand_stack.push_int(3);
    let local_vars = local_vars;
    let frame = Frame {
        operand_stack,
        local_vars,
    };

    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}
