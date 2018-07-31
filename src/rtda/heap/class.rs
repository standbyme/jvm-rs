use classfile::class_file::ClassFile;
use classfile::constant_pool::ConstantPool;
use classfile::member_info::MemberInfo;
use rtda::heap::class_loader::ClassLoader;

use rtda::heap::field::Field;
use rtda::heap::method::Method;
use std::rc::Rc;

pub struct Class {
    pub access_flags: u16,
    pub name: String,
    //    pub super_class_name: String,
    //    interface_names: Vec<String>,
    //    constant_pool: ConstantPool,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    //    loader * ClassLoader
    pub super_class: Option<Rc<Class>>,
    //    interfaces        [] * Class
    //    instanceSlotCount uint
    //    staticSlotCount   uint
    //    staticVars        Slots,
}

impl Class {
    pub fn main_method(&self) -> &Method {
        self.get_method("main", "([Ljava/lang/String;)V", true)
    }
    fn get_method(&self, name: &str, descriptor: &str, is_static: bool) -> &Method {
        println!(
            "name {} descriptor {} is_static {}",
            name, descriptor, is_static
        );
        self.methods
            .iter()
            .find(|x| {
                x.is_static() == is_static && x.name() == name && x.descriptor() == descriptor
            })
            .expect("Method not found")
    }
}
