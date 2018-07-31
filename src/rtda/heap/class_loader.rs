use classfile::class_file::ClassFile;
use classfile::class_reader::ClassReader;
use classfile::member_info::MemberInfo;
use classpath::classpath::ClassPath;
use rtda::heap::class::Class;
use rtda::heap::field::Field;
use rtda::heap::method::Method;
use rtda::slot::Slot;
use std::collections::HashMap;
use std::rc::Rc;
use vec_map::VecMap;

pub struct ClassLoader {
    class_path: ClassPath,
    class_map: HashMap<String, Rc<Class>>,
}

struct Acc {
    next_instance_field_slot_id: usize,
    next_static_field_slot_id: usize,
    static_vars: VecMap<Slot>,
}

impl ClassLoader {
    pub fn new(class_path: ClassPath) -> ClassLoader {
        ClassLoader {
            class_path,
            class_map: HashMap::new(),
        }
    }

    pub fn load(self, name: String) -> (Rc<Class>, ClassLoader) {
        println!("load {}", &name);
        if self.class_map.contains_key(&name) {
            let class = Rc::clone(self.class_map.get(&name).unwrap());
            (class, self)
        } else {
            let data = self.read(&name);
            let (class, mut class_loader) = ClassLoader::define(self, data);
            let class_copy = Rc::clone(&class);
            class_loader.class_map.insert(name, class);
            (class_copy, class_loader)
        }
    }

    fn read(&self, name: &str) -> Vec<u8> {
        self.class_path
            .read_class(name)
            .expect("java.lang.ClassNotFoundException")
    }

    fn define(class_loader: ClassLoader, data: Vec<u8>) -> (Rc<Class>, ClassLoader) {
        let class_file = data.parse();
        let name = class_file.class_name().to_owned();
        let super_class_name = class_file.super_class_name().to_owned();
        let ClassFile {
            access_flags,
            methods,
            fields,
            ..
        } = class_file;

        let methods: Vec<Method> = methods.into_iter().map(|x| Method::new(x)).collect();

        let (super_class, class_loader) = if name != "java/lang/Object" {
            let (class, class_loader) = class_loader.load(super_class_name);
            (Some(class), class_loader)
        } else {
            (None, class_loader)
        };
        let fields: Vec<Field> = fields.into_iter().map(|x| Field::new(x)).collect();
        let class = Rc::new(Class {
            access_flags,
            fields,
            name,
            super_class,
            methods,
        });

        (class, class_loader)
    }
}
