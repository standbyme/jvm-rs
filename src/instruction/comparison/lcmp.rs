use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn LCMP(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    let Frame {
        operand_stack,
        local_vars,
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
    };
    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[cfg(test)]
mod tests {
    use instruction::comparison::lcmp::*;
    use instruction::instruction::ExecuteResult;
    use rtda::frame::Frame;
    use rtda::operand_stack::OperandStack;
    use rtda::vars::Vars;
    use util::code_reader::CodeReader;

    #[test]
    #[allow(non_snake_case)]
    fn test_LCMP_gt() {
        let frame = create_frame(9223372036854775807i64, 9223372036854775806i64);
        let (ExecuteResult { frame, offset: _ }, _) = LCMP(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 1);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_LCMP_lt() {
        let frame = create_frame(-9223372036854775806i64, 9223372036854775807i64);
        let (ExecuteResult { frame, offset: _ }, _) = LCMP(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, -1);
    }
    #[test]
    #[allow(non_snake_case)]
    fn test_LCMP_eq() {
        let frame = create_frame(-9223372036854775806i64, -9223372036854775806i64);
        let (ExecuteResult { frame, offset: _ }, _) = LCMP(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 0);
    }

    fn create_frame(op1: i64, op2: i64) -> Frame {
        let operand_stack = OperandStack::new(10);
        let operand_stack = operand_stack.push_long(op1);
        let operand_stack = operand_stack.push_long(op2);
        Frame {
            local_vars: Vars::new(10),
            operand_stack: operand_stack,
        }
    }
}
