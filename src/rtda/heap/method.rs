use rtda::heap::class_member::ClassMember;

pub struct Method {
    class_member: ClassMember,
    max_locals: usize,
    max_stack: usize,
    code: Vec<u8>,
}
