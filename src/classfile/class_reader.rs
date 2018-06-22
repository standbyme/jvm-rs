extern crate byteorder;
extern crate vec_map;

use self::byteorder::{ByteOrder, BigEndian};
use self::vec_map::VecMap;

use classfile::attribute_info;
use classfile::attribute_info::ExceptionTableEntry;
use classfile::constant_info::ConstantInfo;
use classfile::constant_pool::ConstantPool;
use classfile::member_info::MemberInfo;
use classfile::attribute_info::AttributeInfo;
use util::modified_utf8::from_modified_utf8;


const CONSTANT_UTF8: u8 = 1;
const CONSTANT_INTEGER: u8 = 3;
const CONSTANT_FLOAT: u8 = 4;
const CONSTANT_LONG: u8 = 5;
const CONSTANT_DOUBLE: u8 = 6;
const CONSTANT_CLASS: u8 = 7;
const CONSTANT_STRING: u8 = 8;
const CONSTANT_FIELDREF: u8 = 9;
const CONSTANT_METHODREF: u8 = 10;
const CONSTANT_INTERFACE_METHODREF: u8 = 11;
const CONSTANT_NAME_AND_TYPE: u8 = 12;

#[derive(Debug)]
pub struct ClassFile {
    pub major_version: u16,
    pub minor_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<MemberInfo>,
    pub methods: Vec<MemberInfo>,
    pub attributes: Vec<Box<dyn AttributeInfo>>,
}

#[derive(Debug)]
pub struct VersionInfo {
    major_version: u16,
    minor_version: u16,
}

pub trait ClassReader {
    fn read_u8(&self) -> (u8, &[u8]);
    fn read_u16(&self) -> (u16, &[u8]);
    fn read_u16s(&self) -> (Vec<u16>, &[u8]);
    fn read_u32(&self) -> (u32, &[u8]);
    fn read_i32(&self) -> (i32, &[u8]);
    fn read_f32(&self) -> (f32, &[u8]);
    fn read_i64(&self) -> (i64, &[u8]);
    fn read_f64(&self) -> (f64, &[u8]);
    fn read_bytes(&self, n: usize) -> (&[u8], &[u8]);
    fn read_and_check_magic(&self) -> (u32, &[u8]);
    fn read_and_check_version(&self) -> (VersionInfo, &[u8]);
    fn read_constant_info(&self) -> (ConstantInfo, &[u8]);
    fn read_constant_pool(&self) -> (ConstantPool, &[u8]);
    fn read_access_flags(&self) -> (u16, &[u8]);
    fn read_this_class(&self) -> (u16, &[u8]);
    fn read_super_class(&self) -> (u16, &[u8]);
    fn read_interfaces(&self) -> (Vec<u16>, &[u8]);
    fn read_member(&self, constant_pool: &ConstantPool) -> (MemberInfo, &[u8]);
    fn read_members(&self, constant_pool: &ConstantPool) -> (Vec<MemberInfo>, &[u8]);
    fn read_exception_table(&self) -> (Vec<ExceptionTableEntry>, &[u8]);
    fn read_attribute(&self, constant_pool: &ConstantPool) -> (Box<dyn AttributeInfo>, &[u8]);
    fn read_attributes(&self, constant_pool: &ConstantPool) -> (Vec<Box<dyn AttributeInfo>>, &[u8]);
    fn parse(&self) -> ClassFile;
}

impl ClassReader for [u8] {
    fn read_u8(&self) -> (u8, &[u8]) {
        let (a, b) = self.split_at(1);
        (a[0], b)
    }

    fn read_u16(&self) -> (u16, &[u8]) {
        let (a, b) = self.split_at(2);
        (BigEndian::read_u16(a), b)
    }

    fn read_u16s(&self) -> (Vec<u16>, &[u8]) {
        let (n, after_n) = self.read_u16();
        let mut s: Vec<u16> = Vec::with_capacity(n as usize);
        let mut rest = after_n;
        for _ in 1..=n {
            let (value, next_rest) = rest.read_u16();
            s.push(value);
            rest = next_rest;
        }
        (s, rest)
    }

    fn read_u32(&self) -> (u32, &[u8]) {
        let (a, b) = self.split_at(4);
        (BigEndian::read_u32(a), b)
    }

    fn read_i32(&self) -> (i32, &[u8]) {
        let (a, b) = self.split_at(4);
        (BigEndian::read_i32(a), b)
    }

    fn read_f32(&self) -> (f32, &[u8]) {
        let (a, b) = self.split_at(4);
        (BigEndian::read_f32(a), b)
    }

    fn read_i64(&self) -> (i64, &[u8]) {
        let (a, b) = self.split_at(8);
        (BigEndian::read_i64(a), b)
    }

    fn read_f64(&self) -> (f64, &[u8]) {
        let (a, b) = self.split_at(8);
        (BigEndian::read_f64(a), b)
    }

    fn read_bytes(&self, n: usize) -> (&[u8], &[u8]) {
        self.split_at(n)
    }

    fn read_and_check_magic(&self) -> (u32, &[u8]) {
        let result = self.read_u32();
        let (magic, _) = result;
        assert_eq!(magic, 0xCAFEBABE);
        result
    }

    fn read_and_check_version(&self) -> (VersionInfo, &[u8]) {
        let (minor_version, after_minor_version) = self.read_u16();
        let (major_version, after_major_version) = after_minor_version.read_u16();
        assert_eq!(major_version, 52);
        assert_eq!(minor_version, 0);
        let version_info = VersionInfo {
            major_version,
            minor_version,
        };
        (version_info, after_major_version)
    }

    fn read_constant_info(&self) -> (ConstantInfo, &[u8]) {
        let (tag, after_tag) = self.read_u8();
        match tag {
            CONSTANT_INTEGER => {
                let (val, rest) = after_tag.read_i32();
                (ConstantInfo::Integer(val), rest)
            }
            CONSTANT_FLOAT => {
                let (val, rest) = after_tag.read_f32();
                (ConstantInfo::Float(val), rest)
            }
            CONSTANT_LONG => {
                let (val, rest) = after_tag.read_i64();
                (ConstantInfo::Long(val), rest)
            }
            CONSTANT_DOUBLE => {
                let (val, rest) = after_tag.read_f64();
                (ConstantInfo::Double(val), rest)
            }
            CONSTANT_UTF8 => {
                let (length, after_length) = after_tag.read_u16();
                let (bytes, rest) = after_length.read_bytes(length as usize);
                let string: String = from_modified_utf8(bytes).unwrap();
                (ConstantInfo::UTF8(string), rest)
            }
            CONSTANT_STRING => {
                let (val, rest) = after_tag.read_u16();
                (ConstantInfo::String(val), rest)
            }
            CONSTANT_CLASS => {
                let (val, rest) = after_tag.read_u16();
                (ConstantInfo::Class(val), rest)
            }
            CONSTANT_NAME_AND_TYPE => {
                let (name_index, after_name_index) = after_tag.read_u16();
                let (descriptor_index, rest) = after_name_index.read_u16();
                (ConstantInfo::NameAndType(name_index, descriptor_index), rest)
            }
            CONSTANT_FIELDREF => {
                let (class_index, after_class_index) = after_tag.read_u16();
                let (name_and_type_index, rest) = after_class_index.read_u16();
                (ConstantInfo::FieldRef(class_index, name_and_type_index), rest)
            }
            CONSTANT_METHODREF => {
                let (class_index, after_class_index) = after_tag.read_u16();
                let (name_and_type_index, rest) = after_class_index.read_u16();
                (ConstantInfo::MethodRef(class_index, name_and_type_index), rest)
            }
            CONSTANT_INTERFACE_METHODREF => {
                let (class_index, after_class_index) = after_tag.read_u16();
                let (name_and_type_index, rest) = after_class_index.read_u16();
                (ConstantInfo::InterfaceMethodRef(class_index, name_and_type_index), rest)
            }
            _ => {
                panic!("Wrong tag type");
            }
        }
    }

    fn read_constant_pool(&self) -> (ConstantPool, &[u8]) {
        let (count, after_count) = self.read_u16();
        let mut constant_pool: VecMap<ConstantInfo> = VecMap::with_capacity((count + 1) as usize);

        let mut i: usize = 1;
        let mut rest: &[u8] = after_count;

        while i < (count as usize) {
            let (constant_info, next_rest) = rest.read_constant_info();
            rest = next_rest;
            let add = match constant_info {
                ConstantInfo::Long(_) | ConstantInfo::Double(_) => 2,
                _ => 1
            };
            constant_pool.insert(i, constant_info);
            i = i + add;
        }

        (constant_pool, rest)
    }

    fn read_access_flags(&self) -> (u16, &[u8]) {
        self.read_u16()
    }

    fn read_this_class(&self) -> (u16, &[u8]) {
        self.read_u16()
    }

    fn read_super_class(&self) -> (u16, &[u8]) {
        self.read_u16()
    }

    fn read_interfaces(&self) -> (Vec<u16>, &[u8]) {
        self.read_u16s()
    }

    fn read_member(&self, constant_pool: &ConstantPool) -> (MemberInfo, &[u8]) {
        let (access_flags, after_access_flags) = self.read_u16();
        let (name_index, after_name_index) = after_access_flags.read_u16();
        let (descriptor_index, after_descriptor_index) = after_name_index.read_u16();
        let (attributes, after_attributes) = after_descriptor_index.read_attributes(constant_pool);
        let member_info = MemberInfo { access_flags, name_index, descriptor_index, attributes };
        (member_info, after_attributes)
    }

    fn read_members(&self, constant_pool: &ConstantPool) -> (Vec<MemberInfo>, &[u8]) {
        let (count, after_count) = self.read_u16();

        let mut members: Vec<MemberInfo> = Vec::with_capacity(count as usize);
        let mut rest = after_count;

        for _ in 1..=count {
            let (member, after_member) = rest.read_member(constant_pool);
            members.push(member);
            rest = after_member;
        }

        (members, rest)
    }

    fn read_exception_table(&self) -> (Vec<ExceptionTableEntry>, &[u8]) {
        let (exception_table_length, after_exception_table_length) = self.read_u16();
        let mut exception_table: Vec<ExceptionTableEntry> = Vec::with_capacity(exception_table_length as usize);
        let mut rest = after_exception_table_length;
        for _ in 1..=exception_table_length {
            let (start_pc, after_start_pc) = rest.read_u16();
            let (end_pc, after_end_pc) = after_start_pc.read_u16();
            let (handler_pc, after_handler_pc) = after_end_pc.read_u16();
            let (catch_type, after_catch_type) = after_handler_pc.read_u16();
            let exception_table_entry = ExceptionTableEntry { start_pc, end_pc, handler_pc, catch_type };
            exception_table.push(exception_table_entry);
            rest = after_catch_type;
        }
        (exception_table, rest)
    }

    fn read_attribute(&self, constant_pool: &ConstantPool) -> (Box<dyn AttributeInfo>, &[u8]) {
        let (attribute_name_index, after_attribute_name_index) = self.read_u16();
        let attribute_name = match constant_pool.get(attribute_name_index as usize).unwrap() {
            ConstantInfo::UTF8(attribute_name) => attribute_name,
            _ => panic!("attribute_name isn't UTF8")
        };
        let (attribute_length, after_attribute_length) = after_attribute_name_index.read_u32();

        match attribute_name.as_str() {
            "Code" => {
                let (max_stack, after_max_stack) = after_attribute_length.read_u16();
                let (max_locals, after_max_locals) = after_max_stack.read_u16();
                let (code_length, after_code_length) = after_max_locals.read_u32();
                let (code, after_code) = after_code_length.read_bytes(code_length as usize);
                let (exception_table, after_exception_table) = after_code.read_exception_table();
                let (attributes, after_attributes) = after_exception_table.read_attributes(constant_pool);

                (Box::new(attribute_info::Code { max_stack, max_locals, code: code.to_vec(), exception_table, attributes }), after_attributes)
            }
            "ConstantValue" => {
                let (constantvalue_index, after_constantvalue_index) = after_attribute_length.read_u16();
                (Box::new(attribute_info::ConstantValue { constantvalue_index }), after_constantvalue_index)
            }
            "Deprecated" => (Box::new(attribute_info::Deprecated {}), after_attribute_length),
            "Exceptions" => {
                let (exception_index_table, after_exception_index_table) = after_attribute_length.read_u16s();
                (Box::new(attribute_info::Exceptions { exception_index_table }), after_exception_index_table)
            }
            "SourceFile" => {
                let (sourcefile_index, after_sourcefile_index) = after_attribute_length.read_u16();
                (Box::new(attribute_info::SourceFile { sourcefile_index }), after_sourcefile_index)
            }
            "Synthetic" => (Box::new(attribute_info::Synthetic {}), after_attribute_length),
            _ => {
                let (_, after_attribute_info) = after_attribute_length.read_bytes(attribute_length as usize);
                let attribute_name = attribute_name.to_string();
                (Box::new(attribute_info::Unparsed { attribute_name, attribute_length }), after_attribute_info)
            }
        }
    }

    fn read_attributes(&self, constant_pool: &ConstantPool) -> (Vec<Box<dyn AttributeInfo>>, &[u8]) {
        let (attributes_count, after_attributes_count) = self.read_u16();
        let mut attributes: Vec<Box<dyn AttributeInfo>> = Vec::with_capacity(attributes_count as usize);
        let mut rest = after_attributes_count;
        for _ in 1..=attributes_count {
            let (attribute_info, next_rest) = rest.read_attribute(constant_pool);
            attributes.push(attribute_info);
            rest = next_rest;
        }
        (attributes, rest)
    }

    fn parse(&self) -> ClassFile {
        let (_, after_magic) = self.read_and_check_magic();
        let (version_info, after_version_info) = after_magic.read_and_check_version();
        let VersionInfo { major_version, minor_version } = version_info;
        let (constant_pool, after_constant_pool) = after_version_info.read_constant_pool();
        let (access_flags, after_access_flags) = after_constant_pool.read_access_flags();
        let (this_class, after_this_class) = after_access_flags.read_this_class();
        let (super_class, after_super_class) = after_this_class.read_this_class();
        let (interfaces, after_interfaces) = after_super_class.read_interfaces();
        let (fields, after_fields) = after_interfaces.read_members(&constant_pool);
        let (methods, after_methods) = after_fields.read_members(&constant_pool);
        let (attributes, _) = after_methods.read_attributes(&constant_pool);
        ClassFile {
            major_version,
            minor_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        }
    }
}

#[cfg(test)]
mod tests {
    use classfile::class_reader::ClassFile;
    use classfile::class_reader::ClassReader;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn parse() {
        let path: &str = "src/test_data/Object.class";
        let input = File::open(path).unwrap();
        let bytes: Vec<u8> = input.bytes().map(|x| x.unwrap()).collect();
        let ClassFile {
            major_version,
            minor_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        } = bytes.parse();
        assert_eq!(major_version, 52);
        assert_eq!(minor_version, 0);
        assert_eq!(access_flags, 33);
        assert_eq!(this_class, 17);
        assert_eq!(super_class,0);
    }
}