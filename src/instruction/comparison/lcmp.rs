use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use rtda::thread::Thread;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn LCMP(code_reader: CodeReader, thread: Thread) -> (ExecuteResult, CodeReader) {
    let (frame, thread) = thread.pop_frame();
    let Frame {
        operand_stack,
        local_vars,
        method,
    } = frame;

    let (val2, operand_stack) = operand_stack.pop_long();
    let (val1, operand_stack) = operand_stack.pop_long();

    let operand_stack = if val1 < val2 {
        operand_stack.push_int(-1)
    } else if val1 > val2 {
        operand_stack.push_int(1)
    } else {
        operand_stack.push_int(0)
    };

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
    use instruction::comparison::lcmp::*;
    use instruction::instruction::ExecuteResult;
    use rtda::frame::Frame;
    use rtda::heap::method::Method;
    use rtda::operand_stack::OperandStack;
    use rtda::thread::Thread;
    use rtda::vars::Vars;
    use std::rc::Rc;
    use util::code_reader::CodeReader;

    #[test]
    #[allow(non_snake_case)]
    fn test_LCMP_gt() {
        let frame = create_frame(9223372036854775807i64, 9223372036854775806i64);
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            LCMP(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 1);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_LCMP_lt() {
        let frame = create_frame(-9223372036854775806i64, 9223372036854775807i64);
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            LCMP(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, -1);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_LCMP_eq() {
        let frame = create_frame(-9223372036854775806i64, -9223372036854775806i64);
        let thread = Thread::new().push_frame(frame);
        let (ExecuteResult { thread, offset: _ }, _) =
            LCMP(CodeReader::new(Rc::new(vec![])), thread);
        let (frame, _) = thread.pop_frame();
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 0);
    }

    fn create_frame(op1: i64, op2: i64) -> Frame {
        let operand_stack = OperandStack::new(10);
        let operand_stack = operand_stack.push_long(op1);
        let operand_stack = operand_stack.push_long(op2);
        let method = Rc::new(Method::new(MemberInfo {
            access_flags: 0u16,
            name: "".to_string(),
            name_index: 0u16,
            descriptor_index: 0u16,
            descriptor: "".to_string(),
            attributes: vec![],
        }));
        Frame {
            local_vars: Vars::new(10),
            operand_stack,
            method,
        }
    }
}
