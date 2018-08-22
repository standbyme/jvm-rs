use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use rtda::thread::Thread;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
fn _dcmp(frame: Frame, flag: bool) -> (f64, f64, Frame) {
    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;

    let (val2, operand_stack) = operand_stack.pop_double();
    let (val1, operand_stack) = operand_stack.pop_double();

    let operand_stack = if val1 > val2 {
        operand_stack.push_int(1)
    } else if val1 < val2 {
        operand_stack.push_int(-1)
    } else if val1 == val2 {
        operand_stack.push_int(0)
    } else if flag {
        operand_stack.push_int(1)
    } else {
        operand_stack.push_int(-1)
    };

    let frame = Frame {
        class,
        operand_stack,
        local_vars,
        method,
    };
    (val1, val2, frame)
}

#[allow(non_snake_case)]
pub fn DCMPG(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("DCMPG");

    let (frame, thread) = thread.pop_frame();
    let (_, _, frame) = _dcmp(frame, true);
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn DCMPL(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("DCMPG");

    let (frame, thread) = thread.pop_frame();
    let (_, _, frame) = _dcmp(frame, false);
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset: 0 };
    (execute_result, code_reader)
}

#[cfg(test)]
mod tests {
    use classfile::constant_pool::ConstantPool;
    use classfile::member_info::MemberInfo;
    use instruction::comparison::dcmp::*;
    use instruction::instruction::ExecuteResult;
    use rtda::frame::Frame;
    use rtda::heap::class::Class;
    use rtda::heap::method::Method;
    use rtda::operand_stack::OperandStack;
    use rtda::thread::Thread;
    use rtda::vars::Vars;
    use std::rc::Rc;
    use util::code_reader::CodeReader;
    use vec_map::VecMap;

    #[test]
    #[allow(non_snake_case)]
    fn test_DCMPL() {
        let frame = create_frame(1.48, 1.49);
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            DCMPL(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, -1);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_DCMPG() {
        let frame = create_frame(1.49, 1.48);
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            DCMPG(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 1);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_DCMPG_equal() {
        let frame = create_frame(1.49, 1.49);
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            DCMPG(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 0);
    }

    fn create_frame(op1: f64, op2: f64) -> Frame {
        let operand_stack = OperandStack::new(10);
        let operand_stack = operand_stack.push_double(op1);
        let operand_stack = operand_stack.push_double(op2);
        let method = Rc::new(Method::new(MemberInfo {
            access_flags: 0u16,
            name: "".to_string(),
            name_index: 0u16,
            descriptor_index: 0u16,
            descriptor: "".to_string(),
            attributes: vec![],
        }));
        let class = Rc::new(Class {
            access_flags: 0u16,
            name: "".to_string(),
            constant_pool: ConstantPool {
                vec_map: VecMap::new(),
            },
            fields: Vec::new(),
            methods: Vec::new(),
            super_class: None,
            instance_slot_count: 0usize,
            static_slot_count: 0usize,
            static_vars: Vars::new(2),
        });
        Frame {
            class,
            local_vars: Vars::new(10),
            operand_stack: operand_stack,
            method,
        }
    }
}
