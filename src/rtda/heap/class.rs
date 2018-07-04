use classfile::class_file::ClassFile;
use classfile::constant_pool::ConstantPool;
use rtda::heap::field::Field;
use rtda::heap::method::Method;

pub struct Class {
    access_flags: u16,
    name: String,
    super_class_name: String,
    interface_names: Vec<String>,
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

//impl Class {
//    fn new(class_file: ClassFile) -> Class {
//        let ClassFile {
//            access_flags,
//            super_class_name,
//        } = class_file;
//        Class {
//            access_flags,
//
//            super_class_name,
//        }
//    }
//}
