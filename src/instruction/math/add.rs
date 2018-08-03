use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn IADD(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("IADD");

    let Frame {
        operand_stack,
        local_vars,
    } = frame;

    let (v2, operand_stack) = operand_stack.pop_int();
    let (v1, operand_stack) = operand_stack.pop_int();
    let result = v1 + v2;
    let operand_stack = operand_stack.push_int(result);

    let frame = Frame {
        operand_stack,
        local_vars,
    };
    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn DADD(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("DADD");

    let Frame {
        operand_stack,
        local_vars,
    } = frame;

    let (v2, operand_stack) = operand_stack.pop_double();
    let (v1, operand_stack) = operand_stack.pop_double();
    let result = v1 + v2;
    let operand_stack = operand_stack.push_double(result);

    let frame = Frame {
        operand_stack,
        local_vars,
    };
    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn LADD(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("LADD");

    let Frame {
        operand_stack,
        local_vars,
    } = frame;

    let (v2, operand_stack) = operand_stack.pop_long();
    let (v1, operand_stack) = operand_stack.pop_long();
    let result = v1 + v2;
    let operand_stack = operand_stack.push_long(result);

    let frame = Frame {
        operand_stack,
        local_vars,
    };
    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn FADD(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("FADD");

    let Frame {
        operand_stack,
        local_vars,
    } = frame;

    let (v2, operand_stack) = operand_stack.pop_float();
    let (v1, operand_stack) = operand_stack.pop_float();
    let result = v1 + v2;
    let operand_stack = operand_stack.push_float(result);

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
    use instruction::math::add::*;
    use rtda::frame::Frame;
    use util::code_reader::CodeReader;

    #[test]
    #[allow(non_snake_case)]
    fn test_IADD() {
        let frame = Frame::new(1, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(2);
        let operand_stack = operand_stack.push_int(3);

        let frame = Frame {
            operand_stack,
            local_vars,
        };

        let (ExecuteResult { frame, offset: _ }, _) = IADD(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 5);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_DADD() {
        let frame = Frame::new(1, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_double(2.71828182845f64);
        let operand_stack = operand_stack.push_double(3.1415926535897926f64);

        let frame = Frame {
            operand_stack,
            local_vars,
        };

        let (ExecuteResult { frame, offset: _ }, _) = DADD(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_double();
        assert_eq!(val, 5.8598744820397926);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_FADD() {
        let frame = Frame::new(1, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_float(3.1415926);
        let operand_stack = operand_stack.push_float(3.1415926);

        let frame = Frame {
            operand_stack,
            local_vars,
        };

        let (ExecuteResult { frame, offset: _ }, _) = FADD(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_float();
        assert_eq!(val, 6.2831852);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_LADD() {
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

        let (ExecuteResult { frame, offset: _ }, _) = DADD(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_long();
        assert_eq!(val, 15343603549);
    }
}
