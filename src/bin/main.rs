extern crate jvm;

use jvm::classpath::classpath::parse;
use jvm::instruction::instruction;
use jvm::instruction::instruction::ExecuteResult;
use jvm::rtda::frame::Frame;
use jvm::rtda::heap::class_loader::ClassLoader;
use jvm::rtda::heap::method::Method;
use jvm::rtda::thread::Thread;
use jvm::shell::command::Command;
use std::rc::Rc;
use jvm::rtda::heap::class::Class;

fn main() {
    let class_name = "src.test_data.MyObject";
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
    interpret(main_class, main_method)
}

fn interpret(class: Rc<Class>, method: Rc<Method>) {
    let thread = Thread::new();
    let frame = Frame::new(class, method);
    let thread = thread.push_frame(frame);
    execute(thread);
}

fn execute(thread: Thread) {
    let mut mut_pc = 0usize;
    let mut mut_thread = thread;
    while !mut_thread.is_stack_empty() {
        let thread = mut_thread;
        let pc = mut_pc;

        let (execute_result, after_execute) = instruction::execute(mut_pc, thread);
        let ExecuteResult { thread, offset } = execute_result;

        mut_pc = match offset {
            0 => after_execute.pc,
            i => (pc as isize + i) as usize,
        };
        mut_thread = thread;

        println!("pc: {}", pc);
        println!("offset: {}", offset);
        println!("mut_pc: {}", mut_pc);
    }
}
