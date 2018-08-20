use instruction::instruction::ExecuteResult;
use rtda::thread::Thread;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn GOTO(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("GOTO");

    let (offset, code_reader) = code_reader.read_i16();
    let offset = offset as isize;

    let execute_result = ExecuteResult { thread, offset };
    (execute_result, code_reader)
}
