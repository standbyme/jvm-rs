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
    use instruction::instruction::ExecuteResult;
    use instruction::math::and::*;
    use rtda::frame::Frame;
    use util::code_reader::CodeReader;

    #[test]
    #[allow(non_snake_case)]
    fn test_IAND() {
        let frame = Frame::new(1, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(350);
        let operand_stack = operand_stack.push_int(678);

        let frame = Frame {
            operand_stack,
            local_vars,
        };

        let (ExecuteResult { frame, offset: _ }, _) = IAND(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 6);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_LAND() {
        let frame = Frame::new(1, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_long(12345678969);
        let operand_stack = operand_stack.push_long(2997924580);

        let frame = Frame {
            operand_stack,
            local_vars,
        };

        let (ExecuteResult { frame, offset: _ }, _) = LAND(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_long();
        assert_eq!(val, 2458914912);
    }
}
