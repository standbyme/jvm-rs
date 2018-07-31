use classfile::member_info::MemberInfo;
use rtda::heap::class_member::ClassMember;

pub struct Field {
    class_member: ClassMember,
}

impl Field {
    pub fn new(member_info: MemberInfo) -> Field {
        let class_member = ClassMember::new(&member_info);
        Field { class_member }
    }
}
