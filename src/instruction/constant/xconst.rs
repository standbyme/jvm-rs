use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use rtda::thread::Thread;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn DCONST_0(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    // println!("DCONST_0");
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;
    let operand_stack = operand_stack.push_double(0f64);
    let local_vars = local_vars;
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
pub fn DCONST_1(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    // println!("DCONST_1");
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;
    let operand_stack = operand_stack.push_double(1f64);
    let local_vars = local_vars;
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
pub fn FCONST_0(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    // println!("FCONST_0");
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;
    let operand_stack = operand_stack.push_float(0f32);
    let local_vars = local_vars;
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
pub fn FCONST_1(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    // println!("FCONST_1");
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;
    let operand_stack = operand_stack.push_float(1f32);
    let local_vars = local_vars;
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
pub fn FCONST_2(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    // println!("FCONST_2");
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;
    let operand_stack = operand_stack.push_float(2f32);
    let local_vars = local_vars;
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
pub fn ICONST_M1(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    // println!("ICONST_M1");

    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;
    let operand_stack = operand_stack.push_int(-1);
    let local_vars = local_vars;
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
pub fn ICONST_0(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    // println!("ICONST_0");
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;
    let operand_stack = operand_stack.push_int(0);
    let local_vars = local_vars;
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
pub fn ICONST_1(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    // println!("ICONST_1");
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;
    let operand_stack = operand_stack.push_int(1);
    let local_vars = local_vars;
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
pub fn ICONST_2(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    // println!("ICONST_2");
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;
    let operand_stack = operand_stack.push_int(2);
    let local_vars = local_vars;
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
pub fn ICONST_3(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    // println!("ICONST_3");
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;
    let operand_stack = operand_stack.push_int(3);
    let local_vars = local_vars;
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
pub fn ICONST_4(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    // println!("ICONST_4");
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;
    let operand_stack = operand_stack.push_int(4);
    let local_vars = local_vars;
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
pub fn ICONST_5(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    // println!("ICONST_5");
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;
    let operand_stack = operand_stack.push_int(5);
    let local_vars = local_vars;
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
pub fn LCONST_0(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    // println!("LCONST_0");
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;
    let operand_stack = operand_stack.push_long(0i64);
    let local_vars = local_vars;
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
pub fn LCONST_1(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    // println!("LCONST_1");
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
        class,
    } = frame;
    let operand_stack = operand_stack.push_long(1i64);
    let local_vars = local_vars;
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
mod tests {
    use classfile::constant_pool::ConstantPool;
    use classfile::member_info::MemberInfo;
    use instruction::constant::xconst::*;
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
    fn test_DCONST_0() {
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
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            DCONST_0(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_double();
        assert_eq!(val, 0f64);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_DCONST_1() {
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
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            DCONST_1(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_double();
        assert_eq!(val, 1f64);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_FCONST_0() {
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
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            FCONST_0(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_float();
        assert_eq!(val, 0f32);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_FCONST_1() {
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
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            FCONST_1(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_float();
        assert_eq!(val, 1f32);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_FCONST_2() {
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
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            FCONST_2(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_float();
        assert_eq!(val, 2f32);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ICONST_M1() {
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
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            ICONST_M1(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, -1);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ICONST_0() {
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
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            ICONST_0(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ICONST_1() {
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
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            ICONST_1(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 1);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ICONST_2() {
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
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            ICONST_2(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 2);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ICONST_3() {
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
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            ICONST_3(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 3);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ICONST_4() {
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
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            ICONST_4(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 4);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ICONST_5() {
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
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            ICONST_5(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 5);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_LCONST_0() {
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
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            LCONST_0(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_long();
        assert_eq!(val, 0i64);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_LCONST_1() {
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
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            LCONST_1(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_long();
        assert_eq!(val, 1i64);
    }
}
