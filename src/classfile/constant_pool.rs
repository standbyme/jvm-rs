use vec_map::VecMap;
use std::ops::Index;
use classfile::constant_info::ConstantInfo;

#[derive(Debug)]
pub struct ConstantPool {
    pub vec_map: VecMap<ConstantInfo>,
}

impl ConstantPool {
    pub fn insert(&mut self, index: usize, constant_info: ConstantInfo) {
        self.vec_map.insert(index, constant_info);
    }

    pub fn get(&self, index: usize) -> &ConstantInfo {
        self.vec_map.get(index).unwrap()
    }

    pub fn capacity(&self) -> usize {
        self.vec_map.capacity()
    }
}