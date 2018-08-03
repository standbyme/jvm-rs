use classfile::attribute_info::AttributeInfo;
use classfile::constant_pool::ConstantPool;
use classfile::member_info::MemberInfo;

#[derive(Debug)]
pub struct ClassFile {
    pub major_version: u16,
    pub minor_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<MemberInfo>,
    pub methods: Vec<MemberInfo>,
    pub attributes: Vec<AttributeInfo>,
}

impl ClassFile {
    pub fn main_method<'a>(&'a self) -> &'a MemberInfo {
        self.methods
            .iter()
            .find(|x| x.name == "main" && x.descriptor == "([Ljava/lang/String;)V")
            .expect("Main method not found")
    }

    pub fn class_name(&self) -> &str {
        self.constant_pool.get_class_name(self.this_class as usize)
    }

    pub fn super_class_name(&self) -> &str {
        let super_class = self.super_class as usize;
        if super_class > 0 {
            self.constant_pool.get_class_name(super_class)
        } else {
            ""
        }
    }
}
