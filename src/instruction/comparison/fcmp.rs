use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use util::code_reader::CodeReader;

#[allow(non_snake_case)]
fn _fcmp(frame: Frame, flag: bool) -> (f32, f32, Frame) {
    let Frame {
        operand_stack,
        local_vars,
    } = frame;

    let (val2, operand_stack) = operand_stack.pop_float();
    let (val1, operand_stack) = operand_stack.pop_float();

    let operand_stack = if val1 < val2 {
        operand_stack.push_int(-1)
    } else if val1 > val2 {
        operand_stack.push_int(1)
    } else if val1 == val2 {
        operand_stack.push_int(0)
    } else if flag {
        operand_stack.push_int(1)
    } else {
        operand_stack.push_int(-1)
    };

    let frame = Frame {
        operand_stack,
        local_vars,
    };
    (val1, val2, frame)
}

#[allow(non_snake_case)]
pub fn FCMPG(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("FCMPG");

    let (_, _, frame) = _fcmp(frame, true);
    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn FCMPL(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("FCMPL");

    let (_, _, frame) = _fcmp(frame, false);
    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[cfg(test)]
mod tests {
    use instruction::comparison::fcmp::*;
    use rtda::frame::Frame;
    use rtda::local_vars::LocalVars;
    use rtda::operand_stack::OperandStack;
    use util::code_reader::CodeReader;

    #[test]
    #[allow(non_snake_case)]
    fn test_FCMPL() {
        let frame = create_frame(0f32, 1f32);
        let (ExecuteResult { frame, offset: _ }, _) = FCMPL(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, -1);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_FCMPG() {
        let frame = create_frame(2f32, 1f32);
        let (ExecuteResult { frame, offset: _ }, _) = FCMPG(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 1);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_FCMPG_equal() {
        let frame = create_frame(2.345f32, 2.345f32);
        let (ExecuteResult { frame, offset: _ }, _) = FCMPG(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 0);
    }

    fn create_frame(op1: f32, op2: f32) -> Frame {
        let os = OperandStack::new(10);
        let os = os.push_float(op1);
        let os = os.push_float(op2);
        Frame {
            local_vars: LocalVars::new(10),
            operand_stack: os,
        }
    }
}
