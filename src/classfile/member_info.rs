use classfile::attribute_info::AttributeInfo;

#[derive(Debug)]
pub struct MemberInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<AttributeInfo>,
}