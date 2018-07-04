use instruction::instruction::ExecuteResult;
use rtda::frame::Frame;
use util::code_reader::CodeReader;
use std::f64;

#[allow(non_snake_case)]
fn _dcmp(frame: Frame, flag: bool) -> (f64, f64, Frame) {
    let Frame {
        operand_stack,
        local_vars,
    } = frame;

    let (val2, operand_stack) = operand_stack.pop_double();
    let (val1, operand_stack) = operand_stack.pop_double();
    
    let operand_stack = if val1 > val2 {
        operand_stack.push_int(1)
    } else if val1 < val2 {
        operand_stack.push_int(-1)
    } else if val1 == val2 {
        operand_stack.push_int(0) 
    } else if flag{
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
pub fn DCMPG(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("DCMPG");

    let (_, _, frame) = _dcmp(frame, true);
    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}

#[allow(non_snake_case)]
pub fn DCMPL(code_reader: CodeReader, frame: Frame) -> (ExecuteResult, CodeReader) {
    println!("DCMPG");

    let (_, _, frame) = _dcmp(frame, false);
    let execute_result = ExecuteResult { frame, offset: 0 };
    (execute_result, code_reader)
}



#[cfg(test)]
mod tests {
    use instruction::comparison::dcmp::*; 
    use rtda::frame::Frame;
    use rtda::local_vars::LocalVars;
    use rtda::operand_stack::OperandStack;
    use util::code_reader::CodeReader;


    #[test]
    #[allow(non_snake_case)]
    fn test_DCMPL() {
        let frame = create_frame(1.48,1.49);
        let (ExecuteResult {frame, offset: _}, _) = DCMPL(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, -1); 
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_DCMPG() {
        let frame = create_frame(1.49,1.48);
        let (ExecuteResult {frame, offset: _}, _) = DCMPG(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 1); 
    }

     #[test]
    #[allow(non_snake_case)]
    fn test_DCMPG_equal() {
        let frame = create_frame(1.49,1.49);
        let (ExecuteResult {frame, offset: _}, _) = DCMPG(CodeReader::new(&vec![]), frame);
        let (val, _) = frame.operand_stack.pop_int();
        assert_eq!(val, 0); 
    }



    fn create_frame(op1: f64, op2: f64) -> Frame {
        let operand_stack = OperandStack::new(10);
        let operand_stack = operand_stack.push_double(op1);
        let operand_stack = operand_stack.push_double(op2);
        Frame {
            local_vars: LocalVars::new(10),
            operand_stack: operand_stack,
        }
    }


}
