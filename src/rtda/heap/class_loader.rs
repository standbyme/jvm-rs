use classpath::classpath::ClassPath;
use rtda::heap::class::Class;
use std::collections::HashMap;

struct ClassLoader {
    class_path: ClassPath,
    class_map: HashMap<String, Class>,
}

impl ClassLoader {
    fn new(class_path: ClassPath) -> ClassLoader {
        ClassLoader {
            class_path,
            class_map: HashMap::new(),
        }
    }
//
//    fn load_class(&mut self, name: &str) -> &Class {
//        match self.class_map.get(name) {
//            Some(val) => val,
//            None => {}
//        }
//    }
//
//    fn load_new_class(&mut self, name: &str) -> &Class {
//        let data = self.read_class(name);
//    }
//
//    fn read_class(&self, name: &str) -> Vec<u8> {
//        self.class_path
//            .read_class(name)
//            .expect("java.lang.ClassNotFoundException")
//    }
//
//    fn define_class(data: Vec<u8>) -> Class {
//        let class_file = data.parse();
//    }
}
