use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use util::code_reader::CodeReader;


#[allow(non_snake_case)]
pub fn DMUL(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    
    println!("DMUL");
    
    let Frame{
        operand_stack,
        local_vars,
    } = frame;

    let (v2, operand_stack) = operand_stack.pop_double();
    let (v1, operand_stack) = operand_stack.pop_double();
    let result = v1 * v2; 
    let operand_stack = operand_stack.push_double(result);

    let frame = Frame {
        operand_stack,
        local_vars,
    }; 
    let execute_result = ExecuteResult { frame, offset: 0};
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn FMUL(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    
    println!("FMUL");
    
    let Frame{
        operand_stack,
        local_vars,
    } = frame;

    let (v2, operand_stack) = operand_stack.pop_float();
    let (v1, operand_stack) = operand_stack.pop_float();
    let result = v1 * v2; 
    let operand_stack = operand_stack.push_float(result);

    let frame = Frame {
        operand_stack,
        local_vars,
    }; 
    let execute_result = ExecuteResult { frame, offset: 0};
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn IMUL(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    
    println!("IMUL");
    
    let Frame{
        operand_stack,
        local_vars,
    } = frame;

    let (v2, operand_stack) = operand_stack.pop_int();
    let (v1, operand_stack) = operand_stack.pop_int();
    let result = v1 * v2; 
    let operand_stack = operand_stack.push_int(result);

    let frame = Frame {
        operand_stack,
        local_vars,
    }; 
    let execute_result = ExecuteResult { frame, offset: 0};
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn LMUL(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    
    println!("LMUL");
    
    let Frame{
        operand_stack,
        local_vars,
    } = frame;

    let (v2, operand_stack) = operand_stack.pop_long();
    let (v1, operand_stack) = operand_stack.pop_long();
    let result = v1 * v2; 
    let operand_stack = operand_stack.push_long(result);

    let frame = Frame {
        operand_stack,
        local_vars,
    }; 
    let execute_result = ExecuteResult { frame, offset: 0};
    (execute_result, code_reader)
}


#[cfg(test)]
mod test {
     use instruction::math::mul::*;
    use instruction::instruction::ExecuteResult;
    use rtda::frame::Frame;
    use rtda::local_vars::LocalVars;
    use rtda::operand_stack::OperandStack;
    use util::code_reader::CodeReader;

    #[test]
    #[allow(non_snake_case)]
    fn test_DMUL() {
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

        let (ExecuteResult { frame, offset: _ }, _) = DMUL(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_double();
        assert_eq!(val, 8.53973422264514888498427947f64); 
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_FMUL() {
         let frame = Frame::new(1, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_float(2.71828182845f32);
        let operand_stack = operand_stack.push_float(3.1415926535897926f32);

        let frame = Frame {
            operand_stack,
            local_vars,
        };

        let (ExecuteResult { frame, offset: _ }, _) = FMUL(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_float();
        assert_eq!(val, 8.53973422264514888498427947f32); 
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_IMUL() {
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

        let (ExecuteResult { frame, offset: _ }, _) = IMUL(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 6); 
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_LMUL() {
         let frame = Frame::new(1, 2);
        let Frame {
            operand_stack,
            local_vars,
        } = frame;

        let operand_stack = operand_stack.push_long(1234567890);
        let operand_stack = operand_stack.push_long(2997924580);

        let frame = Frame {
            operand_stack,
            local_vars,
        };

        let (ExecuteResult { frame, offset: _ }, _) = LMUL(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_long();
        assert_eq!(val, 3701141423109736200); 
    }


}
