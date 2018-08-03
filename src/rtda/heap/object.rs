use rtda::heap::class::Class;
use rtda::slot::Slot;

struct Object {
    class: Class,
    fields: Vec<Slot>,
}
