use util::code_reader::CodeReader;
use rtda::frame::Frame;
use instruction::instruction::ExecuteResult;

#[allow(non_snake_case)]
pub fn GOTO(reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    let (offset, reader) = reader.read_i16();
    let offset = offset as isize;

    let execute_result = ExecuteResult {
        frame,
        offset,
    };
    (execute_result, reader)
}