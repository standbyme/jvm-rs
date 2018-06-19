#[derive(Debug)]
pub enum ConstantInfo {
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    UTF8(String),
    String(u16),
    Class(u16),
    NameAndType(u16, u16),
    FieldRef(u16,u16),
    MethodRef(u16,u16),
    InterfaceMethodRef(u16,u16),
    Empty
}