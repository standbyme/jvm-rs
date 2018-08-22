use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use rtda::thread::Thread;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn IINC(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("IINC");

    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;

    let (index, code_reader) = code_reader.read_u8();
    let (val_1, code_reader) = code_reader.read_u8();
    let index = index as usize;
    let val_1 = val_1 as i32;

    let val_2 = local_vars.get_int(index);
    let val = val_1 + val_2;
    let local_vars = local_vars.set_int(index, val);

    let frame = Frame {
        class,
        operand_stack,
        local_vars,
        method,
    };
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset: 0 };
    (execute_result, code_reader)
}
