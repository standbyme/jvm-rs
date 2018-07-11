use classfile::class_file::ClassFile;
use classfile::constant_pool::ConstantPool;
use rtda::heap::class_loader::ClassLoader;
use rtda::heap::field::Field;
use rtda::heap::method::Method;

pub struct Class<'a> {
    access_flags: u16,
    pub name: &'a str,
    super_class_name: &'a str,
    //    interface_names: Vec<String>,
    //    constant_pool: ConstantPool,
    //    fields: Field,
    //    methods: Method,
    //    loader * ClassLoader
    //    superClass * Class
    //    interfaces        [] * Class
    //    instanceSlotCount uint
    //    staticSlotCount   uint
    //    staticVars        Slots,
}

//impl<'a> Class<'a> {
//    pub fn new(class_file: ClassFile, class_loader: &ClassLoader) -> Class<'a> {
//        let access_flags = class_file.access_flags;
//        Class {
//            access_flags,
//            name: class_file.get_class_name(),
//            super_class_name: class_file.get_super_class_name(),
//        }
//    }
//}
