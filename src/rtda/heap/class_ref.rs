use rtda::heap::symbol_ref::SymbolRef;

struct ClassRef<'a> {
    symbol_ref: SymbolRef<'a>,
}
