extern crate byteorder;
extern crate vec_map;

use self::byteorder::{ByteOrder, BigEndian};
use self::vec_map::VecMap;

use rtda::slot::Slot;
use util::byte;

pub trait LocalVars {
    fn set_int(self, index: usize, val: i32) -> VecMap<Slot>;
    fn get_int(&self, index: usize) -> i32;
}

impl dyn LocalVars {
    pub fn new(max_locals: usize) -> VecMap<Slot> {
        if max_locals > 0 {
            VecMap::with_capacity(max_locals)
        } else {
            panic!("max_locals < 0")
        }
    }
}

impl LocalVars for VecMap<Slot> {
    fn set_int(mut self, index: usize, val: i32) -> VecMap<Slot> {
        self.insert(index, Slot::Bytes(byte::i32_to_u8seq(val)));
        self
    }

    fn get_int(&self, index: usize) -> i32 {
        match self[index] {
            Slot::Bytes(val) => byte::u8seq_to_i32(val),
            _ => panic!("get_int from wrong place")
        }
    }
}