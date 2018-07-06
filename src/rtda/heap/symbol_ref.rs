use classfile::constant_pool::ConstantPool;
use rtda::heap::class::Class;

pub struct SymbolRef<'a> {
    constant_pool: ConstantPool,
    class_name: String,
    class: Class<'a>,
}
