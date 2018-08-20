use classfile::constant_pool::ConstantPool;
use rtda::heap::field::Field;
use rtda::heap::method::Method;
use rtda::vars::Vars;
use std::rc::Rc;

pub struct Class {
    pub access_flags: u16,
    pub name: String,
    //    pub super_class_name: String,
    //    interface_names: Vec<String>,
    pub constant_pool: ConstantPool,
    pub fields: Vec<Field>,
    pub methods: Vec<Rc<Method>>,
    //    loader * ClassLoader
    pub super_class: Option<Rc<Class>>,
    //    interfaces        [] * Class
    pub instance_slot_count: usize,
    pub static_slot_count: usize,
    pub static_vars: Vars,
}

impl Class {
    pub fn main_method(&self) -> Rc<Method> {
        self.get_method("main", "([Ljava/lang/String;)V", true)
    }
    fn get_method(&self, name: &str, descriptor: &str, is_static: bool) -> Rc<Method> {
        println!(
            "name {} descriptor {} is_static {}",
            name, descriptor, is_static
        );
        let reference = self.methods
            .iter()
            .find(|x| {
                x.is_static() == is_static && x.name() == name && x.descriptor() == descriptor
            })
            .expect("Method not found");
        Rc::clone(reference)
    }
}
