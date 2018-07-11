use rtda::heap::member_ref::MemberRef;

struct FieldRef<'a> {
    member_ref: MemberRef<'a>,
}
