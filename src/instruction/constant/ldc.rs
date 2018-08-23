use classfile::constant_info::ConstantInfo;
use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use rtda::thread::Thread;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn LDC(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("LDC");
    let (index, code_reader) = code_reader.read_u8();
    let (frame, thread) = thread.pop_frame();
    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;
    let class_copy = class.clone();
    let constant_info = class_copy.constant_pool.get(index as usize);
    let operand_stack = match constant_info {
        ConstantInfo::Integer(val) => operand_stack.push_int(*val),
        ConstantInfo::Float(val) => operand_stack.push_float(*val),
        _ => panic!("TODO: LDC"),
    };
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

#[allow(non_snake_case)]
pub fn LDC2_W(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("LDC2_W");
    let (index, code_reader) = code_reader.read_u16();
    let (frame, thread) = thread.pop_frame();
    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;
    let class_copy = class.clone();
    let constant_info = class_copy.constant_pool.get(index as usize);
    let operand_stack = match constant_info {
        ConstantInfo::Long(val) => operand_stack.push_long(*val),
        ConstantInfo::Double(val) => operand_stack.push_double(*val),
        _ => panic!("java.lang.ClassFormatError"),
    };
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
