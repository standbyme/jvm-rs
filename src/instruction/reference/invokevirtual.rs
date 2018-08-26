use instruction::instruction::ExecuteResult;
use rtda::thread::Thread;
use util::code_reader::CodeReader;
use rtda::frame::Frame;

#[allow(non_snake_case)]
pub fn INVOKE_VIRTUAL(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    let (frame, thread) = thread.pop_frame();
    let Frame {
        operand_stack,
        local_vars,
        method,
        class
    } = frame;

    let (x, operand_stack) = operand_stack.pop_int();
    println!("{}", x);
    let frame = Frame {
        operand_stack,
        local_vars,
        method,
        class,
    };
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset: 0 };
    (execute_result, code_reader)
}
