use instruction::instruction::ExecuteResult;
use rtda::thread::Thread;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn RETURN(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    let (_, thread) = thread.pop_frame();
    let execute_result = ExecuteResult { thread, offset: 0 };
    (execute_result, code_reader)
}
