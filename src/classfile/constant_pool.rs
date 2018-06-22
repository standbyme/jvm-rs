extern crate vec_map;

use self::vec_map::VecMap;

use classfile::constant_info::ConstantInfo;

pub type ConstantPool = VecMap<ConstantInfo>;