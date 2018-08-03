extern crate vec_map;

use self::vec_map::VecMap;

use rtda::slot::Slot;

#[derive(Debug)]
pub struct Vars {
    vec_map: VecMap<Slot>,
}

impl Vars {
    pub fn new(max_locals: usize) -> Vars {
        if max_locals > 0 {
            let vec_map = VecMap::with_capacity(max_locals);
            Vars { vec_map }
        } else {
            panic!("max_locals < 0")
        }
    }

    pub fn set_int(mut self, index: usize, val: i32) -> Vars {
        self.vec_map.insert(index, Slot::Num(val));
        self
    }

    pub fn get_int(&self, index: usize) -> i32 {
        match self.vec_map[index] {
            Slot::Num(val) => val,
            _ => panic!("get_int from wrong place"),
        }
    }
}
