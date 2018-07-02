use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
fn _icmpPop(frame: Frame) -> (i32, i32, Frame) {
    let Frame {
        operand_stack,
        local_vars,
    } = frame;

    let (val2, operand_stack) = operand_stack.pop_int();
    let (val1, operand_stack) = operand_stack.pop_int();

    let frame = Frame {
        operand_stack,
        local_vars,
    };
    (val1, val2, frame)
}

#[allow(non_snake_case)]
pub fn IF_ICMPGT(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("IF_ICMPGT");
    let (offset, code_reader) = code_reader.read_i16();

    let (val1, val2, frame) = _icmpPop(frame);
    let offset = if val1 > val2 { offset as isize } else { 0 };

    let execute_result = ExecuteResult { frame, offset };
    (execute_result, code_reader)
}


pub fn IF_ICMPGE(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
	println!("IF_ICMPGE");
    let (offset, code_reader) = code_reader.read_i16();
    
    let (val1, val2, frame) = _icmpPop(frame);
    let offset = if val1 >= val2 { offset as isize } else { 0 };

    let execute_result = ExecuteResult { frame, offset };
    (execute_result, code_reader}

}

pub fn IF_ICMPEQ(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
	println!("IF_ICMPEQ");
    let (offset, code_reader) = code_reader.read_i16();
    
    let (val1, val2, frame) = _icmpPop(frame);
    let offset = if val1 == val2 { offset as isize } else { 0 };

    let execute_result = ExecuteResult { frame, offset };
    (execute_result, code_reader}

}


pub fn IF_ICMPNE(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
	println!("IF_ICMPNE");
    let (offset, code_reader) = code_reader.read_i16();
    
    let (val1, val2, frame) = _icmpPop(frame);
    let offset = if val1 != val2 { offset as isize } else { 0 };

    let execute_result = ExecuteResult { frame, offset };
    (execute_result, code_reader};

}

pub fn IF_ICMPLT(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
	println!("IF_ICMPLT");
    let (offset, code_reader) = code_reader.read_i16();
    
    let (val1, val2, frame) = _icmpPop(frame);
    let offset = if val1 < val2 { offset as isize } else { 0 };

    let execute_result = ExecuteResult { frame, offset };
    (execute_result, code_reader}

}

pub fn IF_ICMPLE(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
	println!("IF_ICMPLE");
    let (offset, code_reader) = code_reader.read_i16();
    
    let (val1, val2, frame) = _icmpPop(frame);
    let offset = if val1 <= val2 { offset as isize } else { 0 };

    let execute_result = ExecuteResult { frame, offset };
    (execute_result, code_reader}

}


