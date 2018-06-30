extern crate jvm;

use jvm::classfile::member_info::MemberInfo;
use jvm::classfile::attribute_info::AttributeInfo;
use jvm::rtda::thread::Thread;
use jvm::rtda::frame::Frame;
use jvm::util::code_reader::CodeReader;

fn main() {
    println!("Start")
}

fn interpret(method_info: MemberInfo) {
    let code_attribute = method_info.get_code_attribute();
    let (max_stack, max_locals, code) = match code_attribute {
        AttributeInfo::Code { max_stack, max_locals, code, exception_table: _, attributes: _ } => (max_stack, max_locals, code),
        _ => panic!("Not Code Attribute")
    };
    let thread = Thread::new();
    let frame = Frame::new(*max_locals as usize, *max_stack as usize);
    let thread = thread.push_frame(frame);
    execute(thread, &code);
}

fn execute(thread: Thread, code: &Vec<u8>) {
    let (frame, thread) = thread.pop_frame();
    let mut code_reader = CodeReader::new(code);
    let pc = 0usize;
    loop {
        let next_code_reader = code_reader.set_pc(pc);
        let (opcode, after_opcode) = next_code_reader.read_u8();
        code_reader = after_opcode;
    }
}