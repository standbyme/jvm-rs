use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use rtda::thread::Thread;
use util::code_reader::CodeReader;

fn _istore(frame: Frame, index: usize) -> Frame {
    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;
    let (val, operand_stack) = operand_stack.pop_int();
    let local_vars = local_vars.set_int(index, val);
    Frame {
        class,
        operand_stack,
        local_vars,
        method,
    }
}

#[allow(non_snake_case)]
pub fn ISTORE_1(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("ISTORE_1");
    let (frame, thread) = thread.pop_frame();

    let frame = _istore(frame, 1);
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn ISTORE_2(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("ISTORE_2");
    let (frame, thread) = thread.pop_frame();

    let frame = _istore(frame, 2);
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset: 0 };
    (execute_result, code_reader)
}
