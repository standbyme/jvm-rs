use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use util::code_reader::CodeReader;



#[allow(non_snake_case)]
fn _dcmp(frame: Frame) -> (f64, f64, Frame) {
    let Frame {
        operand_stack,
        local_vars,
    } = frame;

    let (val2, operand_stack) = operand_stack.pop_double();
    let (val1, operand_stack) = operand_stack.pop_double();

    let frame = Frame {
        operand_stack,
        local_vars,
    };
    (val1, val2, frame) 
}


pub fn DCMPG(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("DCMPG"); 
    let (offset, code_reader) = code_reader.read_i16();

    let(val1, val2, frame) = _dcmp(frame);
    let offset = if val1 > val2 {offset as isize} else {0};

    let execute_result = ExecuteResult{ frame, offset };
    (execute_result, code_reader)
}

pub fn DCMPL(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("DCMPG"); 
    let (offset, code_reader) = code_reader.read_i16();

    let(val1, val2, frame) = _dcmp(frame);
    let offset = if val1 < val2 {offset as isize} else {0};

    let execute_result = ExecuteResult{ frame, offset };
    (execute_result, code_reader)
}


