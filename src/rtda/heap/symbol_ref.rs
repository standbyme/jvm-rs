use classfile::constant_pool::ConstantPool;
use rtda::heap::class::Class;

pub struct SymbolRef {
    constant_pool: ConstantPool,
    class_name: String,
    class: Class,
}
