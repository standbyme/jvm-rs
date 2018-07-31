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
    pub fn code_attribute(&self) -> Option<&AttributeInfo> {
        self.attributes.iter().find(|x| match x {
            AttributeInfo::Code { .. } => true,
            _ => false,
        })
    }
}
