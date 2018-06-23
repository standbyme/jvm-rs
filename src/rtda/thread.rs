use rtda::stack::Stack;
use rtda::frame::Frame;

const STACK_SIZE: usize = 1024;

struct Thread {
    pc: isize,
    stack: Stack,
}

impl Thread {
    fn new() -> Thread {
        Thread {
            pc: 0,
            stack: Stack::new(STACK_SIZE),
        }
    }

    fn push_frame(self, frame: Frame) -> Thread {
        let Thread { pc, stack } = self;
        Thread {
            pc,
            stack: stack.push(frame),
        }
    }

    fn pop_frame(self) -> (Frame, Thread) {
        let Thread { pc, stack } = self;
        let (frame, stack) = stack.pop();
        let thread = Thread {
            pc,
            stack,
        };
        (frame, thread)
    }
}