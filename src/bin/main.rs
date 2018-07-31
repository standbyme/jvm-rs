extern crate jvm;

use jvm::classpath::classpath::parse;
use jvm::instruction::instruction;
use jvm::instruction::instruction::ExecuteResult;
use jvm::rtda::frame::Frame;
use jvm::rtda::heap::class_loader::ClassLoader;
use jvm::rtda::heap::method::Method;
use jvm::rtda::thread::Thread;
use jvm::shell::command::Command;
use jvm::util::code_reader::CodeReader;

fn main() {
    let class_name = "src.test_data.GaussTest";
    let class_name = class_name.replace('.', "/");
    let command = Command {
        class_name,
        cp_opt: None,
        jre_opt: None,
        args: vec![],
    };
    start_jvm(command);
}

fn start_jvm(command: Command) {
    let class_path = parse(command.jre_opt, command.cp_opt);
    let class_loader = ClassLoader::new(class_path);
    let (main_class, _) = class_loader.load(command.class_name);
    let main_method = main_class.main_method();
    interpret(main_method)
}

fn interpret(method: &Method) {
    let Method {
        max_stack,
        max_locals,
        code,
        ..
    } = method;

    let thread = Thread::new();
    let frame = Frame::new(*max_locals as usize, *max_stack as usize);
    let thread = thread.push_frame(frame);
    execute(thread, &code);
}

fn execute(thread: Thread, code: &Vec<u8>) {
    let (frame, _) = thread.pop_frame();
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
            i => (pc as isize + i) as usize,
        };

        println!("pc: {}", pc);
        println!("offset: {}", offset);
        println!("mut_pc: {}", mut_pc);
        println!();
        mut_frame = frame;
        mut_code_reader = after_execute;
    }
}
