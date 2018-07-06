use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
pub fn DCONST_0(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("DCONST_0");
    let Frame {
        operand_stack,
        local_vars,
    } = frame;
    let operand_stack = operand_stack.push_double(0f64);
    let local_vars = local_vars;
    let frame = Frame {
        operand_stack,
        local_vars,
    };

    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)] 
pub fn DCONST_1(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("DCONST_1");
    let Frame {
        operand_stack,
        local_vars,
    } = frame;
    let operand_stack = operand_stack.push_double(1f64);
    let local_vars = local_vars;
    let frame = Frame {
        operand_stack,
        local_vars,
    };

    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn FCONST_0(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("FCONST_0");
    let Frame {
        operand_stack,
        local_vars,
    } = frame;
    let operand_stack = operand_stack.push_float(0f32);
    let local_vars = local_vars;
    let frame = Frame {
        operand_stack,
        local_vars,
    };

    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn FCONST_1(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("FCONST_1");
    let Frame {
        operand_stack,
        local_vars,
    } = frame;
    let operand_stack = operand_stack.push_float(1f32);
    let local_vars = local_vars;
    let frame = Frame {
        operand_stack,
        local_vars,
    };

    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn FCONST_2(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("FCONST_2");
    let Frame {
        operand_stack,
        local_vars,
    } = frame;
    let operand_stack = operand_stack.push_float(2f32);
    let local_vars = local_vars;
    let frame = Frame {
        operand_stack,
        local_vars,
    };

    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn ICONST_M1(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("ICONST_M1");

    let Frame {
        operand_stack,
        local_vars,
    } = frame;
    let operand_stack = operand_stack.push_int(-1);
    let local_vars = local_vars;
    let frame = Frame {
        operand_stack,
        local_vars,
    };

    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
} 

#[allow(non_snake_case)]
pub fn ICONST_0(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("ICONST_0");

    let Frame {
        operand_stack,
        local_vars,
    } = frame;
    let operand_stack = operand_stack.push_int(0);
    let local_vars = local_vars;
    let frame = Frame {
        operand_stack,
        local_vars,
    };

    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn ICONST_1(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("ICONST_1");

    let Frame {
        operand_stack,
        local_vars,
    } = frame;
    let operand_stack = operand_stack.push_int(1);
    let local_vars = local_vars;
    let frame = Frame {
        operand_stack,
        local_vars,
    };

    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn ICONST_2(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("ICONST_2");

    let Frame {
        operand_stack,
        local_vars,
    } = frame;
    let operand_stack = operand_stack.push_int(2);
    let local_vars = local_vars;
    let frame = Frame {
        operand_stack,
        local_vars,
    };

    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn ICONST_3(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("ICONST_3");

    let Frame {
        operand_stack,
        local_vars,
    } = frame;
    let operand_stack = operand_stack.push_int(3);
    let local_vars = local_vars;
    let frame = Frame {
        operand_stack,
        local_vars,
    };

    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn ICONST_4(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("ICONST_4");

    let Frame {
        operand_stack,
        local_vars,
    } = frame;
    let operand_stack = operand_stack.push_int(4);
    let local_vars = local_vars;
    let frame = Frame {
        operand_stack,
        local_vars,
    };

    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
} 

#[allow(non_snake_case)]
pub fn ICONST_5(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("ICONST_5");

    let Frame {
        operand_stack,
        local_vars,
    } = frame;
    let operand_stack = operand_stack.push_int(5);
    let local_vars = local_vars;
    let frame = Frame {
        operand_stack,
        local_vars,
    };

    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
} 

#[allow(non_snake_case)]
pub fn LCONST_0(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("LCONST_0");

    let Frame {
        operand_stack,
        local_vars,
    } = frame;
    let operand_stack = operand_stack.push_long(0i64);
    let local_vars = local_vars;
    let frame = Frame {
        operand_stack,
        local_vars,
    };

    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
} 

#[allow(non_snake_case)]
pub fn LCONST_1(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("LCONST_1");

    let Frame {
        operand_stack,
        local_vars,
    } = frame;
    let operand_stack = operand_stack.push_long(1i64);
    let local_vars = local_vars;
    let frame = Frame {
        operand_stack,
        local_vars,
    };

    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
} 

#[cfg(test)]
mod tests {
    use instruction::constant::xconst::*;
    use instruction::instruction::ExecuteResult;
    use rtda::frame::Frame;
    use util::code_reader::CodeReader;

    #[test]
    #[allow(non_snake_case)]
    fn test_DCONST_0() {
        let frame = Frame::new(10, 10);
        let (ExecuteResult { frame, offset: _ }, _) = DCONST_0(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_double();
        assert_eq!(val, 0f64);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_DCONST_1() {
        let frame = Frame::new(10, 10);
        let (ExecuteResult { frame, offset: _ }, _) = DCONST_1(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_double();
        assert_eq!(val, 1f64);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_FCONST_0() {
        let frame = Frame::new(10, 10);
        let (ExecuteResult { frame, offset: _ }, _) = FCONST_0(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_float();
        assert_eq!(val, 0f32);
    } 

    #[test]
    #[allow(non_snake_case)]
    fn test_FCONST_1() {
        let frame = Frame::new(10, 10);
        let (ExecuteResult { frame, offset: _ }, _) = FCONST_1(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_float();
        assert_eq!(val, 1f32);
    } 

    #[test]
    #[allow(non_snake_case)]
    fn test_FCONST_2() {
        let frame = Frame::new(10, 10);
        let (ExecuteResult { frame, offset: _ }, _) = FCONST_2(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_float();
        assert_eq!(val, 2f32);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ICONST_M1() {
        let frame = Frame::new(10, 10);
        let (ExecuteResult { frame, offset: _ }, _) = ICONST_M1(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, -1);
    } 

    #[test]
    #[allow(non_snake_case)]
    fn test_ICONST_0() {
        let frame = Frame::new(10, 10);
        let (ExecuteResult { frame, offset: _ }, _) = ICONST_0(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ICONST_1() {
        let frame = Frame::new(10, 10);
        let (ExecuteResult { frame, offset: _ }, _) = ICONST_1(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 1);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ICONST_2() {
        let frame = Frame::new(10, 10);
        let (ExecuteResult { frame, offset: _ }, _) = ICONST_2(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 2);
    } 

    #[test]
    #[allow(non_snake_case)]
    fn test_ICONST_3() {
        let frame = Frame::new(10, 10);
        let (ExecuteResult { frame, offset: _ }, _) = ICONST_3(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 3);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ICONST_4() {
        let frame = Frame::new(10, 10);
        let (ExecuteResult { frame, offset: _ }, _) = ICONST_4(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 4);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ICONST_5() {
        let frame = Frame::new(10, 10);
        let (ExecuteResult { frame, offset: _ }, _) = ICONST_5(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 5);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_LCONST_0() {
        let frame = Frame::new(10, 10);
        let (ExecuteResult { frame, offset: _ }, _) = LCONST_0(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_long();
        assert_eq!(val, 0i64);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_LCONST_1() {
        let frame = Frame::new(10, 10);
        let (ExecuteResult { frame, offset: _ }, _) = LCONST_1(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_long();
        assert_eq!(val, 1i64);
    } 
}
