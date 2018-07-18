use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn DNEG(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    let Frame {
        operand_stack,
        local_vars,
    } = frame;

    let (v, operand_stack) = operand_stack.pop_double();
    let operand_stack = operand_stack.push_double(-v);
    let frame = Frame {
        operand_stack,
        local_vars
    };

    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn FNEG(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    let Frame {
        operand_stack,
        local_vars,
    } = frame;

    let (v, operand_stack) = operand_stack.pop_float();
    let operand_stack = operand_stack.push_float(-v);
    let frame = Frame {
        operand_stack,
        local_vars
    };

    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn INEG(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    let Frame {
        operand_stack,
        local_vars,
    } = frame;

    let (v, operand_stack) = operand_stack.pop_int();
    let operand_stack = operand_stack.push_int(-v);
    let frame = Frame {
        operand_stack,
        local_vars
    };

    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn LNEG(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    let Frame {
        operand_stack,
        local_vars,
    } = frame;

    let (v, operand_stack) = operand_stack.pop_long();
    let operand_stack = operand_stack.push_long(-v);
    let frame = Frame {
        operand_stack,
        local_vars
    };

    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[cfg(test)]
mod tests {
    use instruction::math::neg::*;
    use instruction::instruction::ExecuteResult;
    use rtda::frame::Frame;
    use rtda::local_vars::LocalVars;
    use rtda::operand_stack::OperandStack;
    use std::f64;
    use std::f32;
    use util::code_reader::CodeReader;

    #[test]
    #[allow(non_snake_case)]
    fn test_DNEG() {
        let frame = Frame::new(1, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_double(2f64);

        let frame = Frame {
            operand_stack,
            local_vars,
        };

        let (ExecuteResult { frame, offset: _ }, _) = DNEG(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_double();
        assert_eq!(val, -2f64);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_DNEG_zero() {
        let frame = Frame::new(1, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_double(-0f64);

        let frame = Frame {
            operand_stack,
            local_vars,
        };

        let (ExecuteResult { frame, offset: _ }, _) = DNEG(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_double();
        assert_eq!(val, 0f64);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_DNEG_inf() {
        let frame = Frame::new(1, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_double(f64::INFINITY);

        let frame = Frame {
            operand_stack,
            local_vars,
        };

        let (ExecuteResult { frame, offset: _ }, _) = DNEG(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_double();
        assert_eq!(val, f64::NEG_INFINITY);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_FNEG() {
        let frame = Frame::new(1, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_float(-100.7678f32);

        let frame = Frame {
            operand_stack,
            local_vars,
        };

        let (ExecuteResult { frame, offset: _ }, _) = FNEG(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_float();
        assert_eq!(val, 100.7678f32);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_FNEG_max_min() {
        let frame = Frame::new(1, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_float(f32::MAX);

        let frame = Frame {
            operand_stack,
            local_vars,
        };

        let (ExecuteResult { frame, offset: _ }, _) = FNEG(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_float();
        assert_eq!(val, f32::MIN);
    }
    #[test]
    #[allow(non_snake_case)]
    fn test_INEG() {
        let frame = Frame::new(1, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(234556);

        let frame = Frame {
            operand_stack,
            local_vars,
        };

        let (ExecuteResult { frame, offset: _ }, _) = INEG(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, -234556);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_LNEG() {
        let frame = Frame::new(1, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_long(-54875845748435i64);

        let frame = Frame {
            operand_stack,
            local_vars,
        };

        let (ExecuteResult { frame, offset: _ }, _) = LNEG(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_long();
        assert_eq!(val, 54875845748435i64);
    }
}
