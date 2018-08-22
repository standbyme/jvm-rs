use classfile::attribute_info::AttributeInfo;
use classfile::member_info::MemberInfo;
use rtda::heap::class_member::ClassMember;

pub struct Field {
    pub class_member: ClassMember,
    pub constant_value_index: Option<usize>,
}

impl Field {
    pub fn new(member_info: MemberInfo) -> Field {
        let class_member = ClassMember::new(&member_info);
        let constant_value_index = member_info.constant_value_attribute().map(|x| match x {
            AttributeInfo::ConstantValue {
                constant_value_index,
            } => *constant_value_index as usize,
            _ => panic!("Not ConstantValue"),
        });
        Field {
            class_member,
            constant_value_index,
        }
    }

    pub fn is_static(&self) -> bool {
        self.class_member.is_static()
    }

    pub fn is_long_or_double(&self) -> bool {
        let descriptor = &self.class_member.descriptor;
        descriptor == "J" || descriptor == "D"
    }

    pub fn is_final(&self) -> bool {
        self.class_member.is_final()
    }
}
