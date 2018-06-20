#[derive(Debug)]
pub struct ExceptionTableEntry {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Debug)]
pub struct CodeAttributeInfo {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
pub enum AttributeInfo {
    Code(CodeAttributeInfo),
    // constantvalue_index
    ConstantValue(u16),
    Deprecated,
    // exception_index_table
    Exceptions(Vec<u16>),
    // sourcefile_index
    SourceFile(u16),
    Synthetic,
    Unparsed(String, u32),
}