use classfile::constant_info::ConstantInfo;
use vec_map::VecMap;

#[derive(Debug)]
pub struct ConstantPool {
    pub vec_map: VecMap<ConstantInfo>,
}

impl ConstantPool {
    pub fn insert(&mut self, index: usize, constant_info: ConstantInfo) {
        self.vec_map.insert(index, constant_info);
    }

    pub fn get(&self, index: usize) -> &ConstantInfo {
        self.vec_map.get(index).expect("Bad constant pool index")
    }

    pub fn capacity(&self) -> usize {
        self.vec_map.capacity()
    }

    fn get_utf8(&self, index: usize) -> &str {
        match self.get(index) {
            ConstantInfo::UTF8(ref name) => name,
            _ => panic!("index isn't to UTF8"),
        }
    }

    pub fn get_class_name(&self, index: usize) -> &str {
        let constant_info = self.get(index);
        let name_index = match constant_info {
            ConstantInfo::Class { name_index } => name_index,
            _ => panic!("index isn't to Class"),
        };
        self.get_utf8(*name_index as usize)
    }
}
