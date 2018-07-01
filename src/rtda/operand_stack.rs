use rtda::slot::Slot;
use util::converter;

#[derive(Debug)]
pub struct OperandStack {
    vec: Vec<Slot>,
}

impl OperandStack {
    pub fn new(max_stack: usize) -> OperandStack {
        OperandStack {
            vec: Vec::with_capacity(max_stack),
        }
    }

    pub fn push_int(mut self, val: i32) -> OperandStack {
        self.vec.push(Slot::Num(val));
        self
    }

    pub fn pop_int(mut self) -> (i32, OperandStack) {
        let val = self.vec.pop().unwrap();
        match val {
            Slot::Num(val) => (val, self),
            _ => panic!(),
        }
    }

    pub fn push_long(mut self, val: i64) -> OperandStack {
        let [a, b] = converter::i64_to_i32seq(val);
        self.vec.push(Slot::Num(a));
        self.vec.push(Slot::Num(b));
        self
    }

    pub fn pop_long(mut self) -> (i64, OperandStack) {
        let b = match self.vec.pop().unwrap() {
            Slot::Num(val) => val,
            _ => panic!(),
        };
        let a = match self.vec.pop().unwrap() {
            Slot::Num(val) => val,
            _ => panic!(),
        };
        (converter::i32seq_to_i64([a, b]), self)
    }

    pub fn push_double(mut self, val: f64) -> OperandStack {
        let [a, b] = converter::f64_to_i32seq(val);
        self.vec.push(Slot::Num(a));
        self.vec.push(Slot::Num(b));
        self
    }

    pub fn pop_double(mut self) -> (f64, OperandStack) {
        let b = match self.vec.pop().unwrap() {
            Slot::Num(val) => val,
            _ => panic!(),
        };
        let a = match self.vec.pop().unwrap() {
            Slot::Num(val) => val,
            _ => panic!(),
        };
        (converter::i32seq_to_f64([a, b]), self)
    }
}
