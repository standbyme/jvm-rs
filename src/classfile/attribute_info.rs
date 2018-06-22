use std::fmt::Debug;

#[derive(Debug)]
pub struct ExceptionTableEntry {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

pub trait AttributeInfo: Debug {}

#[derive(Debug)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attributes: Vec<Box<dyn AttributeInfo>>,
}

impl AttributeInfo for Code {}

#[derive(Debug)]
pub struct ConstantValue {
    pub constantvalue_index: u16
}

impl AttributeInfo for ConstantValue {}

#[derive(Debug)]
pub struct Deprecated {}

impl AttributeInfo for Deprecated {}

#[derive(Debug)]
pub struct Exceptions {
    pub exception_index_table: Vec<u16>
}

impl AttributeInfo for Exceptions {}

#[derive(Debug)]
pub struct SourceFile {
    pub sourcefile_index: u16
}

impl AttributeInfo for SourceFile {}

#[derive(Debug)]
pub struct Synthetic {}

impl AttributeInfo for Synthetic {}

#[derive(Debug)]
pub struct Unparsed {
    pub attribute_name: String,
    pub attribute_length: u32,
}

impl AttributeInfo for Unparsed {}


