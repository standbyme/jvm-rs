use classfile::attribute_info::AttributeInfo;

#[derive(Debug)]
pub struct MemberInfo {
    pub access_flags: u16,
    pub name: String,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub descriptor: String,
    pub attributes: Vec<AttributeInfo>,
}

impl MemberInfo {
    pub fn get_code_attribute(&self) -> &AttributeInfo {
        self
            .attributes
            .iter()
            .find(|x| match x {
                AttributeInfo::Code { max_stack: _, max_locals: _, code: _, exception_table: _, attributes: _ } => true,
                _ => false
            })
            .expect("Code attribute not found")
    }
}