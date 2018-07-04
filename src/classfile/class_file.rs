use classfile::attribute_info::AttributeInfo;
use classfile::constant_info::ConstantInfo;
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
    pub fn get_main_method<'a>(&'a self) -> &'a MemberInfo {
        self.methods
            .iter()
            .find(|x| x.name == "main" && x.descriptor == "([Ljava/lang/String;)V")
            .expect("Main method not found")
    }

    fn get_constant_info(&self, index: usize) -> &ConstantInfo {
        self.constant_pool.get(index)
    }

    fn get_utf8(&self, index: usize) -> &str {
        match self.get_constant_info(index) {
            ConstantInfo::UTF8(ref name) => name,
            _ => panic!("index isn't to UTF8"),
        }
    }

    pub fn get_class_name(&self, index: usize) -> &str {
        let constant_info = self.get_constant_info(index);
        let name_index = match constant_info {
            ConstantInfo::Class { name_index } => name_index,
            _ => panic!("index isn't to Class"),
        };
        self.get_utf8(*name_index as usize)
    }
}
