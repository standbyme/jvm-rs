use rtda::heap::symbol_ref::SymbolRef;
use std::rc::Rc;
use classfile::constant_pool::ConstantPool;
use classfile::constant_info::ConstantInfo;

struct ClassRef {
    symbol_ref: SymbolRef,
}

impl ClassRef {
    fn new(constant_pool: Rc<ConstantPool>, constant_class_info: ConstantInfo) -> ClassRef {
        let symbol_ref = SymbolRef{
            constant_pool,

        }
    }
}