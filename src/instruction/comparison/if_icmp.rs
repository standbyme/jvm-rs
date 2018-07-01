use util::code_reader::CodeReader;
use rtda::frame::Frame;
use instruction::instruction::ExecuteResult;

//#[allow(non_camel_case_types)]
//pub struct IF_ICMPGT {
//    offset: isize
//}
//
//impl Instruction for IF_ICMPGT {
//    fn execute(&self, frame: Frame) -> ExecuteResult {
//        let Frame { operand_stack, local_vars } = frame;
//
//
////        func (self *IF_ICMPGT) Execute(frame *rtda.Frame) {
////            if val1, val2 := _icmpPop(frame); val1 > val2 {
////                base.Branch(frame, self.Offset)
////            }
////        }
//        // todo
//        let frame = Frame { operand_stack, local_vars };
//
//        ExecuteResult {
//            frame,
//            offset: 0,
//        }
//    }
//}
//
//impl IF_ICMPGT {
//    pub fn new(reader: CodeReader) -> (Box<dyn Instruction>, CodeReader) {
//        let (offset, reader) = reader.read_i16();
//        (Box::new(IF_ICMPGT { offset }), reader)
//    }
//}