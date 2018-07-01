#[derive(Debug)]
pub struct ExceptionTableEntry {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Debug)]
pub struct LineNumberTableEntry {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Debug)]
pub struct LocalVariableTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

#[derive(Debug)]
pub enum AttributeInfo {
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_table: Vec<ExceptionTableEntry>,
        attributes: Vec<AttributeInfo>,
    },
    ConstantValue {
        constantvalue_index: u16,
    },
    Deprecated,
    Exceptions {
        exception_index_table: Vec<u16>,
    },
    SourceFile {
        sourcefile_index: u16,
    },
    Synthetic,
    Unparsed {
        attribute_name: String,
        attribute_length: u32,
    },
    LineNumberTable {
        line_number_table: Vec<LineNumberTableEntry>,
    },
    LocalVariableTable {
        local_variable_table: Vec<LocalVariableTableEntry>,
    },
}
