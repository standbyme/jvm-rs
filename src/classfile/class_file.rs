use classfile::constant_pool::ConstantPool;
use classfile::member_info::MemberInfo;
use classfile::attribute_info::AttributeInfo;

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
    fn get_main_method<'a>(&'a self) -> &'a MemberInfo {
        self
            .methods
            .iter()
            .find(|x| x.name == "main" && x.descriptor == "([Ljava/lang/String;)V")
            .expect("Main method not found")
    }
}