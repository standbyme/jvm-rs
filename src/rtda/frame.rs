use rtda::operand_stack::OperandStack;
use rtda::vars::Vars;

#[derive(Debug)]
pub struct Frame {
    pub local_vars: Vars,
    pub operand_stack: OperandStack,
}

impl Frame {
    pub fn new(max_locals: usize, max_stack: usize) -> Frame {
        let local_vars = Vars::new(max_locals);
        let operand_stack = OperandStack::new(max_stack);
        Frame {
            local_vars,
            operand_stack,
        }
    }
}

#[cfg(test)]
mod tests {
    use rtda::frame::Frame;
    use rtda::operand_stack::OperandStack;
    use rtda::vars::Vars;

    #[test]
    fn frame() {
        let frame = Frame::new(100, 100);
        local_vars(frame.local_vars);
        operand_stack(frame.operand_stack);
    }

    fn local_vars(local_vars: Vars) {
        let local_vars = local_vars.set_int(0, 100);
        let local_vars = local_vars.set_int(1, -100);
        assert_eq!(local_vars.get_int(0), 100);
        assert_eq!(local_vars.get_int(1), -100);
    }

    fn operand_stack(operand_stack: OperandStack) {
        let operand_stack = operand_stack.push_int(100);
        let operand_stack = operand_stack.push_double(2.71828182845f64);
        let operand_stack = operand_stack.push_int(-100);
        let operand_stack = operand_stack.push_long(2997924580);
        let operand_stack = operand_stack.push_float(3.1415926);
        let (val, operand_stack) = operand_stack.pop_float();
        assert_eq!(val, 3.1415926);
        let (val, operand_stack) = operand_stack.pop_long();
        assert_eq!(val, 2997924580);
        let (val, operand_stack) = operand_stack.pop_int();
        assert_eq!(val, -100);
        let (val, operand_stack) = operand_stack.pop_double();
        assert_eq!(val, 2.71828182845f64);
        let (val, _) = operand_stack.pop_int();
        assert_eq!(val, 100);
    }
}
