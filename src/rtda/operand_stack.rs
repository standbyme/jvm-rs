use rtda::slot::Slot;
use util::converter;

pub trait OperandStack {
    fn push_int(mut self, val: i32) -> Vec<Slot>;
    fn pop_int(mut self) -> (i32, Vec<Slot>);
    fn push_long(mut self, val: i64) -> Vec<Slot>;
    fn pop_long(mut self) -> (i64, Vec<Slot>);
}

impl dyn OperandStack {
    pub fn new(max_stack: usize) -> Vec<Slot> {
        Vec::with_capacity(max_stack)
    }
}


impl OperandStack for Vec<Slot> {
    fn push_int(mut self, val: i32) -> Vec<Slot> {
        self.push(Slot::Num(val));
        self
    }

    fn pop_int(mut self) -> (i32, Vec<Slot>) {
        let val = self.pop().unwrap();
        match val {
            Slot::Num(val) => (val, self),
            _ => panic!()
        }
    }

    fn push_long(mut self, val: i64) -> Vec<Slot> {
        let [a, b] = converter::i64_to_i32seq(val);
        self.push(Slot::Num(a));
        self.push(Slot::Num(b));
        self
    }

    fn pop_long(mut self) -> (i64, Vec<Slot>) {
        let b = match self.pop().unwrap() {
            Slot::Num(val) => val,
            _ => panic!()
        };
        let a = match self.pop().unwrap() {
            Slot::Num(val) => val,
            _ => panic!()
        };
        (converter::i32seq_to_i64([a, b]), self)
    }
}