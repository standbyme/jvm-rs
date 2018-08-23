use classfile::constant_pool::ConstantPool;
use rtda::heap::class::Class;
use std::rc::Rc;
use classfile::constant_info::ConstantInfo;

pub struct SymbolRef {
    constant_pool: Rc<ConstantPool>,
    class_name: String,
    class: Rc<Class>,
}

impl SymbolRef {
    fn new(constant_pool: Rc<ConstantPool>, constant_class_info: ConstantInfo) -> ClassRef {
        let class_name =
    }
}
