use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use rtda::thread::Thread;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
fn _icmpPop(frame: Frame) -> (i32, i32, Frame) {
    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;

    let (val2, operand_stack) = operand_stack.pop_int();
    let (val1, operand_stack) = operand_stack.pop_int();

    let frame = Frame {
        class,
        operand_stack,
        local_vars,
        method,
    };
    (val1, val2, frame)
}

#[allow(non_snake_case)]
pub fn IF_ICMPGT(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    // println!("IF_ICMPGT");
    let (offset, code_reader) = code_reader.read_i16();

    let (frame, thread) = thread.pop_frame();

    let (val1, val2, frame) = _icmpPop(frame);
    let offset = if val1 > val2 { offset as isize } else { 0 };
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn IF_ICMPGE(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    // println!("IF_ICMPGE");
    let (offset, code_reader) = code_reader.read_i16();

    let (frame, thread) = thread.pop_frame();

    let (val1, val2, frame) = _icmpPop(frame);
    let offset = if val1 >= val2 { offset as isize } else { 0 };
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn IF_ICMPEQ(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    // println!("IF_ICMPEQ");
    let (offset, code_reader) = code_reader.read_i16();

    let (frame, thread) = thread.pop_frame();

    let (val1, val2, frame) = _icmpPop(frame);
    let offset = if val1 == val2 { offset as isize } else { 0 };
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn IF_ICMPNE(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    // println!("IF_ICMPNE");
    let (offset, code_reader) = code_reader.read_i16();

    let (frame, thread) = thread.pop_frame();

    let (val1, val2, frame) = _icmpPop(frame);
    let offset = if val1 != val2 { offset as isize } else { 0 };
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn IF_ICMPLT(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    // println!("IF_ICMPLT");
    let (offset, code_reader) = code_reader.read_i16();

    let (frame, thread) = thread.pop_frame();

    let (val1, val2, frame) = _icmpPop(frame);
    let offset = if val1 < val2 { offset as isize } else { 0 };
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn IF_ICMPLE(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    // println!("IF_ICMPLE");
    let (offset, code_reader) = code_reader.read_i16();

    let (frame, thread) = thread.pop_frame();

    let (val1, val2, frame) = _icmpPop(frame);
    let offset = if val1 <= val2 { offset as isize } else { 0 };
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset };
    (execute_result, code_reader)
}

#[cfg(test)]
mod tests {
    use classfile::constant_pool::ConstantPool;
    use classfile::member_info::MemberInfo;
    use instruction::comparison::if_icmp::*;
    use instruction::instruction::ExecuteResult;
    use rtda::frame::Frame;
    use rtda::heap::class::Class;
    use rtda::heap::method::Method;
    use rtda::thread::Thread;
    use rtda::vars::Vars;
    use std::rc::Rc;
    use util::code_reader::CodeReader;
    use vec_map::VecMap;

    #[test]
    #[allow(non_snake_case)]
    fn test_IF_ICMPGT_success() {
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

        let operand_stack = operand_stack.push_int(1);
        let operand_stack = operand_stack.push_int(0);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IF_ICMPGT(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 257);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IF_ICMPGT_fail() {
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

        let operand_stack = operand_stack.push_int(1);
        let operand_stack = operand_stack.push_int(2);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IF_ICMPGT(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IF_ICMPGE_success() {
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

        let operand_stack = operand_stack.push_int(1);
        let operand_stack = operand_stack.push_int(1);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IF_ICMPGE(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 257);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IF_ICMPGE_fail() {
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

        let operand_stack = operand_stack.push_int(0);
        let operand_stack = operand_stack.push_int(2);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IF_ICMPGE(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IF_ICMPEQ_success() {
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

        let operand_stack = operand_stack.push_int(-1);
        let operand_stack = operand_stack.push_int(-1);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IF_ICMPEQ(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 257);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IF_ICMPEQ_fail() {
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

        let operand_stack = operand_stack.push_int(0);
        let operand_stack = operand_stack.push_int(1);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IF_ICMPEQ(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IF_ICMPNE_success() {
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

        let operand_stack = operand_stack.push_int(0);
        let operand_stack = operand_stack.push_int(1);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IF_ICMPNE(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 257);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IF_ICMPNE_fail() {
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

        let operand_stack = operand_stack.push_int(-1);
        let operand_stack = operand_stack.push_int(-1);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IF_ICMPNE(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IF_ICMPLT_success() {
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

        let operand_stack = operand_stack.push_int(1);
        let operand_stack = operand_stack.push_int(2);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IF_ICMPLT(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 257);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IF_ICMPLT_fail() {
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

        let operand_stack = operand_stack.push_int(0);
        let operand_stack = operand_stack.push_int(0);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IF_ICMPLT(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IF_ICMPLE_success() {
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

        let operand_stack = operand_stack.push_int(0);
        let operand_stack = operand_stack.push_int(0);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IF_ICMPLE(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 257);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IF_ICMPLE_fail() {
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
        let operand_stack = operand_stack.push_int(1);
        let frame = Frame {
            class,
            operand_stack,
            local_vars,
            method,
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread: _, offset }, _) =
            IF_ICMPLE(CodeReader::new(Rc::new(vec![1, 1])), thread);
        assert_eq!(offset, 0);
    }
}
