extern crate vec_map;

use self::vec_map::VecMap;

use rtda::slot::Slot;

pub struct LocalVars {
    vec_map: VecMap<Slot>
}

impl LocalVars {
    pub fn new(max_locals: usize) -> LocalVars {
        if max_locals > 0 {
            let vec_map = VecMap::with_capacity(max_locals);
            LocalVars { vec_map }
        } else {
            panic!("max_locals < 0")
        }
    }

    pub fn set_int(mut self, index: usize, val: i32) -> LocalVars {
        self.vec_map.insert(index, Slot::Num(val));
        self
    }

    pub fn get_int(&self, index: usize) -> i32 {
        match self.vec_map[index] {
            Slot::Num(val) => val,
            _ => panic!("get_int from wrong place")
        }
    }
}