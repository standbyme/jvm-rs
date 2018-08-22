use instruction::instruction::ExecuteResult;
use rtda::thread::Thread;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn LDC(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("LDC");
    let (frame, thread) = thread.pop_frame();
    let Frame {
        operand_stack,
        local_vars,
        method,
        class
    } = frame;

    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset: 0 };
}
