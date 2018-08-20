use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use rtda::thread::Thread;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn IAND(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
    } = frame;

    let (v2, operand_stack) = operand_stack.pop_int();
    let (v1, operand_stack) = operand_stack.pop_int();
    let result = v1 & v2;
    let operand_stack = operand_stack.push_int(result);

    let frame = Frame {
        operand_stack,
        local_vars,
        method,
    };
    let thread = thread.push_frame(frame);
    let execute_result = ExecuteResult { thread, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn LAND(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    let (frame, thread) = thread.pop_frame();

    let Frame {
        operand_stack,
        local_vars,
        method,
    } = frame;

    let (v2, operand_stack) = operand_stack.pop_long();
    let (v1, operand_stack) = operand_stack.pop_long();
    let result = v1 & v2;
    let operand_stack = operand_stack.push_long(result);

    let frame = Frame {
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
    use classfile::member_info::MemberInfo;
    use instruction::instruction::ExecuteResult;
    use instruction::math::and::*;
    use rtda::frame::Frame;
    use rtda::heap::method::Method;
    use rtda::thread::Thread;
    use std::rc::Rc;
    use util::code_reader::CodeReader;

    #[test]
    #[allow(non_snake_case)]
    fn test_IAND() {
        let method = Rc::new(Method::new(MemberInfo {
            access_flags: 0u16,
            name: "".to_string(),
            name_index: 0u16,
            descriptor_index: 0u16,
            descriptor: "".to_string(),
            attributes: vec![],
        }));
        let frame = Frame::new(method);
        let Frame {
            operand_stack,
            local_vars,
            method
        } = frame;

        let operand_stack = operand_stack.push_int(350);
        let operand_stack = operand_stack.push_int(678);

        let frame = Frame {
            operand_stack,
            local_vars,
            method
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            IAND(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 6);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_LAND() {
        let method = Rc::new(Method::new(MemberInfo {
            access_flags: 0u16,
            name: "".to_string(),
            name_index: 0u16,
            descriptor_index: 0u16,
            descriptor: "".to_string(),
            attributes: vec![],
        }));
        let frame = Frame::new(method);
        let Frame {
            operand_stack,
            local_vars,
            method
        } = frame;

        let operand_stack = operand_stack.push_long(12345678969);
        let operand_stack = operand_stack.push_long(2997924580);

        let frame = Frame {
            operand_stack,
            local_vars,
            method
        };
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            LAND(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_long();
        assert_eq!(val, 2458914912);
    }
}
