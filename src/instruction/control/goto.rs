use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn GOTO(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("GOTO");

    let (offset, code_reader) = code_reader.read_i16();
    let offset = offset as isize;

    let execute_result = ExecuteResult { frame, offset };
    (execute_result, code_reader)
}
