use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use util::code_reader::CodeReader;

fn _ifcond(frame: Frame) -> (i32, Frame) {
    let Frame {
        operand_stack,
        local_vars,
    } = frame;

    let (val, operand_stack) = operand_stack.pop_int();
    let frame = Frame {
        operand_stack,
        local_vars,
    };
    (val, frame)
}

#[allow(non_snake_case)]
pub fn IFEQ(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("IFEQ");
    let (offset, code_reader) = code_reader.read_i16();

    let (val, frame) = _ifcond(frame);
    let offset = if val == 0 { offset as isize } else { 0 };

    let execute_result = ExecuteResult { frame, offset };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn IFNE(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("IFNE");
    let (offset, code_reader) = code_reader.read_i16();

    let (val, frame) = _ifcond(frame);
    let offset = if val != 0 { offset as isize } else { 0 };

    let execute_result = ExecuteResult { frame, offset };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn IFLT(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("IFLT");
    let (offset, code_reader) = code_reader.read_i16();

    let (val, frame) = _ifcond(frame);
    let offset = if val < 0 { offset as isize } else { 0 };

    let execute_result = ExecuteResult { frame, offset };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn IFGE(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("IFGE");
    let (offset, code_reader) = code_reader.read_i16();

    let (val, frame) = _ifcond(frame);
    let offset = if val >= 0 { offset as isize } else { 0 };

    let execute_result = ExecuteResult { frame, offset };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn IFGT(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("IFGT");
    let (offset, code_reader) = code_reader.read_i16();

    let (val, frame) = _ifcond(frame);
    let offset = if val > 0 { offset as isize } else { 0 };

    let execute_result = ExecuteResult { frame, offset };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn IFLE(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("IFLE");
    let (offset, code_reader) = code_reader.read_i16();

    let (val, frame) = _ifcond(frame);
    let offset = if val <= 0 { offset as isize } else { 0 };

    let execute_result = ExecuteResult { frame, offset };
    (execute_result, code_reader)
}

#[cfg(test)]
mod tests {
    use instruction::comparison::ifcond::*;
    use instruction::instruction::ExecuteResult;
    use rtda::frame::Frame;
    use util::code_reader::CodeReader;

    #[test]
    #[allow(non_snake_case)]
    fn test_IFEQ_success() {
        let frame = Frame::new(1, 1);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(0);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFEQ(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 257);
    }

    #[allow(non_snake_case)]
    fn test_IFEQ_fail() {
        let frame = Frame::new(1, 1);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(0);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFEQ(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IFNE_success() {
        let frame = Frame::new(1, 1);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(1);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFNE(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 257);
    }

    #[allow(non_snake_case)]
    fn test_IFNE_fail() {
        let frame = Frame::new(1, 1);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(0);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFNE(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IFLT_success() {
        let frame = Frame::new(1, 1);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(-1);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFLT(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 257);
    }

    #[allow(non_snake_case)]
    fn test_IFLT_fail() {
        let frame = Frame::new(1, 1);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(0);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFLT(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IFGE_success() {
        let frame = Frame::new(1, 1);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(0);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFGE(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 257);
    }

    #[allow(non_snake_case)]
    fn test_IFGE_fail() {
        let frame = Frame::new(1, 1);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(-1);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFGE(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IFGT_success() {
        let frame = Frame::new(1, 1);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(1);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFGT(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 257);
    }

    #[allow(non_snake_case)]
    fn test_IFGT_fail() {
        let frame = Frame::new(1, 1);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(0);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFGT(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IFLE_success() {
        let frame = Frame::new(1, 1);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(0);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFLE(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 257);
    }

    #[allow(non_snake_case)]
    fn test_IFLE_fail() {
        let frame = Frame::new(1, 1);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_int(1);
        let frame = Frame {
            operand_stack,
            local_vars,
        };
        let (ExecuteResult { frame: _, offset }, _) = IFLE(CodeReader::new(&vec![1, 1]), frame);
        assert_eq!(offset, 0);
    }
}
