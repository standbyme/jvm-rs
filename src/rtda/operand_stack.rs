use rtda::slot::Slot;
use util::byte;

pub trait OperandStack {
    fn push_int(mut self, val: i32) -> Vec<Slot>;
    fn pop_int(mut self) -> (i32, Vec<Slot>);
}

impl dyn OperandStack {
    pub fn new(max_stack: usize) -> Vec<Slot> {
        Vec::with_capacity(max_stack)
    }
}


impl OperandStack for Vec<Slot> {
    fn push_int(mut self, val: i32) -> Vec<Slot> {
        self.push(Slot::Bytes(byte::i32_to_u8seq(val)));
        self
    }
    fn pop_int(mut self) -> (i32, Vec<Slot>) {
        let val = self.pop().unwrap();
        match val {
            Slot::Bytes(val) => (byte::u8seq_to_i32(val), self),
            _ => panic!()
        }
    }
}