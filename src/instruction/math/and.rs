use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn IAND(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {

    let Frame {
        operand_stack,
        local_vars,
    } = frame;

    let (v2, operand_stack) = operand_stack.pop_int();
    let (v1, operand_stack) = operand_stack.pop_int();
    let result = v1 & v2;
    let operand_stack = operand_stack.push_int(result);

    let frame = Frame {
        operand_stack,
        local_vars,
    };
    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn LAND(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {

    let Frame {
        operand_stack,
        local_vars,
    } = frame;

    let (v2, operand_stack) = operand_stack.pop_long();
    let (v1, operand_stack) = operand_stack.pop_long();
    let result = v1 & v2;
    let operand_stack = operand_stack.push_long(result);

    let frame = Frame {
        operand_stack,
        local_vars,
    };
    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[cfg(test)]
mod tests {
    use instruction::math::and::*;

    use instruction::instruction::ExecuteResult;
    use rtda::frame::Frame;
    use rtda::local_vars::LocalVars;
    use rtda::operand_stack::OperandStack;
    use util::code_reader::CodeReader;

    trait Push {
        fn go_on(self, operand_stack : OperandStack) -> OperandStack;
    }

    impl Push for i64 {
        fn go_on(self, operand_stack : OperandStack) -> OperandStack {
            operand_stack.push_long(self)
        }
    }

    impl Push for i32 {
        fn go_on(self, operand_stack : OperandStack) -> OperandStack {
            operand_stack.push_int(self)
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IAND() {
        let frame = create_frame(3i32, 2i32);
        let (ExecuteResult { frame, offset: _ }, _) = IAND(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 2);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_LAND() {
        let frame = create_frame(3i64, 2i64);
        let (ExecuteResult { frame, offset: _ }, _) = LAND(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_long();
        assert_eq!(val, 2i64);
    }

    fn create_frame<T : Push>(op1: T, op2: T) -> Frame {
        let operand_stack = OperandStack::new(10);
        let operand_stack = op1.go_on(operand_stack);
        let operand_stack = op2.go_on(operand_stack);
        Frame {
            local_vars: LocalVars::new(10),
            operand_stack: operand_stack,
        }
    }
}
