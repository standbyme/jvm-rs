use classpath::classpath::ClassPath;
use rtda::heap::class::Class;
use std::collections::HashMap;

pub struct ClassLoader<'a> {
    class_path: ClassPath,
    class_map: HashMap<String, Class<'a>>,
}

//fn parse_class(data: Vec<u8>) -> Class {
//    let class_file = data.parse();
//    Class::new(class_file, &self)
//}

impl<'a> ClassLoader<'a> {
    fn new(class_path: ClassPath) -> ClassLoader<'a> {
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
    fn read_class(&self, name: &str) -> Vec<u8> {
        self.class_path
            .read_class(name)
            .expect("java.lang.ClassNotFoundException")
    }

    fn resolve_super_class(&mut self, class: Class) {
        if class.name != "java/lang/Object" {}
    }

    //    func resolve_super_class(class *Class) {
    //    if class.name != "java/lang/Object" {
    //    class.superClass = class.loader.LoadClass(class.superClassName)
    //    }
    //    }
    //    func resolveInterfaces(class *Class) {
    //    interfaceCount := len(class.interfaceNames)
    //    if interfaceCount > 0 {
    //    class.interfaces = make([]*Class, interfaceCount)
    //    for i, interfaceName := range class.interfaceNames {
    //    class.interfaces[i] = class.loader.LoadClass(interfaceName)
    //    }
    //    }
    //    }

    //    fn define_class(data: Vec<u8>) -> Class {
    //        let class = parse_class(data);
    //    }
}
