use classfile::class_file::ClassFile;
use classfile::class_reader::ClassReader;
use classfile::constant_info::ConstantInfo;
use classfile::constant_pool::ConstantPool;

use classpath::classpath::ClassPath;
use rtda::heap::class::Class;
use rtda::heap::field::Field;
use rtda::heap::method::Method;

use rtda::vars::Vars;
use std::collections::HashMap;
use std::rc::Rc;

pub struct ClassLoader {
    class_path: ClassPath,
    class_map: HashMap<String, Rc<Class>>,
}

struct Acc {
    constant_pool: ConstantPool,
    next_instance_field_slot_id: usize,
    next_static_field_slot_id: usize,
    static_vars: Vars,
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
            constant_pool,
            ..
        } = class_file;

        let methods: Vec<Rc<Method>> = methods
            .into_iter()
            .map(|x| Rc::new(Method::new(x)))
            .collect();

        let (super_class, class_loader) = if name != "java/lang/Object" {
            let (class, class_loader) = class_loader.load(super_class_name);
            (Some(class), class_loader)
        } else {
            (None, class_loader)
        };

        fn fold_func(acc: Acc, field: &Field) -> Acc {
            let Acc {
                next_instance_field_slot_id: instance_field_slot_id,
                next_static_field_slot_id: static_field_slot_id,
                static_vars,
                constant_pool,
            } = acc;
            let slot_id_delta: usize = if field.is_long_or_double() { 2 } else { 1 };
            let (next_instance_field_slot_id, next_static_field_slot_id, static_vars) =
                if field.is_static() {
                    let static_vars: Vars = if field.is_final() {
                        let constant_value_index = field.constant_value_index;
                        if constant_value_index > 0 {
                            match field.class_member.descriptor.as_str() {
                                // todo: Complete  Z B C S I J D Ljava/lang/String
                                "F" => {
                                    let val = match constant_pool.get(constant_value_index) {
                                        ConstantInfo::Integer(val) => *val,
                                        _ => panic!("Not Integer"),
                                    };
                                    static_vars.set_int(static_field_slot_id, val)
                                }
                                _ => panic!("TODO"),
                            }
                        } else {
                            panic!("constant_value_index < 0");
                        }
                    } else {
                        static_vars
                    };
                    (
                        instance_field_slot_id,
                        static_field_slot_id + slot_id_delta,
                        static_vars,
                    )
                } else {
                    (
                        instance_field_slot_id + slot_id_delta,
                        static_field_slot_id,
                        static_vars,
                    )
                };

            Acc {
                next_instance_field_slot_id,
                next_static_field_slot_id,
                static_vars,
                constant_pool,
            }
        }
        let next_static_field_slot_id: usize = 0;
        let next_instance_field_slot_id: usize = super_class
            .clone()
            .map(|x| x.instance_slot_count)
            .unwrap_or(0);
        let static_vars = Vars::new(10);

        let fields: Vec<Field> = fields.into_iter().map(|x| Field::new(x)).collect();
        let Acc {
            next_instance_field_slot_id: instance_slot_count,
            next_static_field_slot_id: static_slot_count,
            static_vars,
            constant_pool,
        } = fields.iter().fold(
            Acc {
                next_instance_field_slot_id,
                next_static_field_slot_id,
                constant_pool,
                static_vars,
            },
            fold_func,
        );
        let class = Rc::new(Class {
            access_flags,
            fields,
            name,
            super_class,
            methods,
            instance_slot_count,
            static_slot_count,
            static_vars,
            constant_pool,
        });

        (class, class_loader)
    }
}
