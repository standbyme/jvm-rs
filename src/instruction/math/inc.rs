use util::code_reader::CodeReader;
use rtda::frame::Frame;
use instruction::instruction::ExecuteResult;


#[allow(non_snake_case)]
fn IINC(reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    let Frame { operand_stack, local_vars } = frame;

    let (index, reader) = reader.read_u8();
    let (val_1, reader) = reader.read_u8();
    let index = index as usize;
    let val_1 = val_1 as i32;

    let val_2 = local_vars.get_int(index);
    let val = val_1 + val_2;
    let local_vars = local_vars.set_int(index, val);

    let frame = Frame { operand_stack, local_vars };
    let execute_result = ExecuteResult {
        frame,
        offset: 0,
    };
    (execute_result, reader)
}