use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use rtda::thread::Thread;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn BIPUSH(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("BIPUSH");
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
    } = frame;

    let (val, code_reader) = code_reader.read_i8();
    let i = val;
    let operand_stack = operand_stack.push_int(i as i32);

    let frame = Frame {
        operand_stack,
        local_vars,
        method,
    };
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset: 0 };
    (execute_result, code_reader)
}
