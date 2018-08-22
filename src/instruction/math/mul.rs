use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use rtda::thread::Thread;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn DMUL(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("DMUL");

    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;

    let (v2, operand_stack) = operand_stack.pop_double();
    let (v1, operand_stack) = operand_stack.pop_double();
    let result = v1 * v2;
    let operand_stack = operand_stack.push_double(result);

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
pub fn FMUL(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("FMUL");
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;

    let (v2, operand_stack) = operand_stack.pop_float();
    let (v1, operand_stack) = operand_stack.pop_float();
    let result = v1 * v2;
    let operand_stack = operand_stack.push_float(result);

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
pub fn IMUL(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("IMUL");
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;

    let (v2, operand_stack) = operand_stack.pop_int();
    let (v1, operand_stack) = operand_stack.pop_int();
    let result = v1 * v2;
    let operand_stack = operand_stack.push_int(result);

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
pub fn LMUL(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    println!("LMUL");
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;

    let (v2, operand_stack) = operand_stack.pop_long();
    let (v1, operand_stack) = operand_stack.pop_long();
    let result = v1 * v2;
    let operand_stack = operand_stack.push_long(result);

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

#[cfg(test)]
mod test {
    use classfile::member_info::MemberInfo;
    use instruction::instruction::ExecuteResult;
    use instruction::math::mul::*;
    use rtda::frame::Frame;
    use rtda::heap::method::Method;
    use rtda::thread::Thread;
    use std::rc::Rc;
    use util::code_reader::CodeReader;
    use rtda::heap::class::Class;
    use vec_map::VecMap;
    use classfile::constant_pool::ConstantPool;
    use rtda::vars::Vars;

    #[test]
    #[allow(non_snake_case)]
    fn test_DMUL() {
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
        let frame = Frame::new(class, method);
        let Frame {
            operand_stack,
            local_vars,
            method,
            class,
        } = frame;

        let operand_stack = operand_stack.push_double(2.71828182845f64);
        let operand_stack = operand_stack.push_double(3.1415926535897926f64);

        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            DMUL(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_double();
        assert_eq!(val, 8.53973422264514888498427947f64);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_FMUL() {
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
        let frame = Frame::new(class, method);
        let Frame {
            operand_stack,
            local_vars,
            method,
            class,
        } = frame;

        let operand_stack = operand_stack.push_float(2.71828182845f32);
        let operand_stack = operand_stack.push_float(3.1415926535897926f32);

        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            FMUL(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_float();
        assert_eq!(val, 8.53973422264514888498427947f32);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IMUL() {
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
        let frame = Frame::new(class, method);
        let Frame {
            operand_stack,
            local_vars,
            method,
            class,
        } = frame;

        let operand_stack = operand_stack.push_int(2);
        let operand_stack = operand_stack.push_int(3);

        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            IMUL(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 6);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_LMUL() {
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
        let frame = Frame::new(class, method);
        let Frame {
            operand_stack,
            local_vars,
            method,
            class,
        } = frame;

        let operand_stack = operand_stack.push_long(1234567890);
        let operand_stack = operand_stack.push_long(2997924580);

        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            LMUL(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_long();
        assert_eq!(val, 3701141423109736200);
    }

}
