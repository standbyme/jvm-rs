use classfile::member_info::MemberInfo;
use rtda::heap::access_flags::*;

pub struct ClassMember {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: String,
}

impl ClassMember {
    pub fn new(member_info: &MemberInfo) -> ClassMember {
        let MemberInfo {
            access_flags,
            name,
            descriptor,
            ..
        } = member_info;
        ClassMember {
            access_flags: *access_flags,
            name: name.to_string(),
            descriptor: descriptor.to_string(),
        }
    }
    pub fn is_static(&self) -> bool {
        self.access_flags & ACC_STATIC != 0
    }

    pub fn is_final(&self) -> bool {
        self.access_flags & ACC_FINAL != 0
    }
}
