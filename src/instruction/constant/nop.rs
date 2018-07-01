use util::code_reader::CodeReader;
use rtda::frame::Frame;
use instruction::instruction::ExecuteResult;

#[allow(non_snake_case)]
pub fn NOP(reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    let execute_result = ExecuteResult {
        frame,
        offset: 0,
    };
    (execute_result, reader)
}