#[derive(Debug)]
pub enum ConstantInfo {
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    UTF8(String),
    String(u16),
    Class(u16),
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    FieldRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    MethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
}