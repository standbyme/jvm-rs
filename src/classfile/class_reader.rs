extern crate byteorder;

use self::byteorder::{ByteOrder, BigEndian};
use classfile::constant_info::ConstantInfo;
use classfile::constant_pool::ConstantPool;
use util::modified_utf8::from_modified_utf8;
use vec_map::VecMap;

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
const CONSTANT_METHOD_HANDLE: u8 = 15;
const CONSTANT_METHOD_TYPE: u8 = 16;
const CONSTANT_INVOKE_DYNAMI: u8 = 18;

#[derive(Debug)]
pub struct ClassFile {
    major_version: u16,
    minor_version: u16,
    constant_pool: ConstantPool,
}

#[derive(Debug)]
pub struct VersionInfo {
    major_version: u16,
    minor_version: u16,
}

pub trait ClassReader {
    fn read_u8(&self) -> (u8, &[u8]);
    fn read_u16(&self) -> (u16, &[u8]);
    fn read_u32(&self) -> (u32, &[u8]);
    fn read_i32(&self) -> (i32, &[u8]);
    fn read_f32(&self) -> (f32, &[u8]);
    fn read_i64(&self) -> (i64, &[u8]);
    fn read_f64(&self) -> (f64, &[u8]);
    fn read_bytes(&self, n: usize) -> (&[u8], &[u8]);
    fn read_and_check_magic(&self) -> (u32, &[u8]);
    fn read_and_check_version(&self) -> (VersionInfo, &[u8]);
    fn read_constant_info(&self, constant_pool: &ConstantPool) -> (ConstantInfo, &[u8]);
    fn read_constant_pool(&self) -> (ConstantPool, &[u8]);
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

    fn read_constant_info(&self, constant_pool: &ConstantPool) -> (ConstantInfo, &[u8]) {
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
        let mut constant_pool: VecMap<ConstantInfo> = VecMap::with_capacity(count as usize);

        let mut i: usize = 1;
        let mut rest: &[u8] = after_count;

        while i < (count as usize) {
            let (constant_info, next_rest) = rest.read_constant_info(&constant_pool);
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

    fn parse(&self) -> ClassFile {
        let (_, after_magic) = self.read_and_check_magic();
        let (version_info, after_version_info) = after_magic.read_and_check_version();
        let VersionInfo { major_version, minor_version } = version_info;
        let (constant_pool, _) = after_version_info.read_constant_pool();
        ClassFile {
            major_version,
            minor_version,
            constant_pool,
        }
    }
}
