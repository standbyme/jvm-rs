use rtda::frame::Frame;

pub struct Stack {
    max_size: usize,
    vec: Vec<Frame>,
}

impl Stack {
    pub fn new(max_size: usize) -> Stack {
        let vec = Vec::with_capacity(max_size);
        Stack { max_size, vec }
    }

    pub fn push(mut self, frame: Frame) -> Stack {
        if self.vec.len() == self.max_size {
            panic!("java.lang.StackOverflowError")
        } else {
            self.vec.push(frame);
            self
        }
    }

    pub fn pop(mut self) -> (Frame, Stack) {
        let frame = self.vec.pop().unwrap();
        (frame, self)
    }
}
