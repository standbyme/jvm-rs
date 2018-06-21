#[derive(Debug)]
pub enum ConstantInfo {
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    UTF8(String),
    String(u16),
    Class(u16),
    // ConstantInfo::NameAndType(name_index, descriptor_index)
    NameAndType(u16, u16),
    // ConstantInfo::FieldRef(class_index, name_and_type_index)
    FieldRef(u16,u16),
    // ConstantInfo::MethodRef(class_index, name_and_type_index)
    MethodRef(u16,u16),
    // ConstantInfo::InterfaceMethodRef(class_index, name_and_type_index)
    InterfaceMethodRef(u16,u16)
}