extern crate jvm;

use jvm::classfile::class_reader::ClassReader;
use jvm::classfile::member_info::MemberInfo;
use jvm::classfile::attribute_info::AttributeInfo;
use jvm::rtda::thread::Thread;
use jvm::rtda::frame::Frame;
use jvm::util::code_reader::CodeReader;
use jvm::instruction::instruction;
use jvm::instruction::instruction::ExecuteResult;

use std::fs::File;
use std::io::Read;
use std::{thread, time};

fn main() {
    let path: &str = "src/test_data/GaussTest.class";
    let input = File::open(path).unwrap();
    let bytes: Vec<u8> = input.bytes().map(|x| x.unwrap()).collect();
    let class_file = bytes.parse();
    let main_method = class_file.get_main_method();
    interpret(main_method)
}

fn interpret(method_info: &MemberInfo) {
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
    let mut mut_code_reader = CodeReader::new(code);
    let mut mut_frame = frame;
    let mut mut_pc = 0usize;
    loop {
        let code_reader = mut_code_reader;
        let frame = mut_frame;
        let pc = mut_pc;

        let next_code_reader = code_reader.set_pc(pc);
        let (opcode, after_opcode) = next_code_reader.read_u8();

        let (execute_result, after_execute) = instruction::execute(opcode, after_opcode, frame);
        let ExecuteResult { frame, offset } = execute_result;

        mut_pc = match offset {
            0 => after_execute.pc,
            i => (pc as isize + offset) as usize
        };

        println!("pc: {}", pc);
        println!("offset: {}", offset);
        println!("mut_pc: {}", mut_pc);
        println!();
        mut_frame = frame;
        mut_code_reader = after_execute;

//        let ten_millis = time::Duration::from_secs(1);
//        thread::sleep(ten_millis);
    }
}