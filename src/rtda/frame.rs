use rtda::heap::class::Class;
use rtda::heap::method::Method;
use rtda::operand_stack::OperandStack;
use rtda::vars::Vars;
use std::rc::Rc;

#[derive(Debug)]
pub struct Frame {
    pub local_vars: Vars,
    pub operand_stack: OperandStack,
    pub method: Rc<Method>,
    pub class: Rc<Class>,
}

impl Frame {
    pub fn new(class: Rc<Class>, method: Rc<Method>) -> Frame {
        let Method {
            max_stack,
            max_locals,
            ..
        } = *method;

        let local_vars = Vars::new(max_locals);
        let operand_stack = OperandStack::new(max_stack);
        Frame {
            class,
            local_vars,
            operand_stack,
            method,
        }
    }
}

#[cfg(test)]
mod tests {
    use classfile::constant_pool::ConstantPool;
    use classfile::member_info::MemberInfo;
    use rtda::frame::Frame;
    use rtda::heap::class::Class;
    use rtda::heap::method::Method;
    use rtda::operand_stack::OperandStack;
    use rtda::vars::Vars;
    use std::rc::Rc;
    use vec_map::VecMap;

    #[test]
    fn frame() {
        let method = Rc::new(Method::new(MemberInfo {
            access_flags: 0u16,
            name: "".to_string(),
            name_index: 0u16,
            descriptor_index: 0u16,
            descriptor: "".to_string(),
            attributes: Vec::new(),
        }));
        let class = Rc::new(Class {
            access_flags: 0u16,
            name: "".to_string(),
            constant_pool: ConstantPool {
                vec_map: VecMap::new(),
            },
            fields: Vec::new(),
            methods: Vec::new(),
            super_class: None,
            instance_slot_count: 0usize,
            static_slot_count: 0usize,
            static_vars: Vars::new(2),
        });
        let frame = Frame::new(class, method);
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
