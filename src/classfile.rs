extern crate byteorder;

use self::byteorder::{ByteOrder, BigEndian};
use std::vec::Vec;

const CONSTANT_CLASS: u8 = 7;
const CONSTANT_FIELDREF: u8 = 9;
const CONSTANT_METHODREF: u8 = 10;
const CONSTANT_INTERFACE_METHODREF: u8 = 11;
const CONSTANT_STRING: u8 = 8;
const CONSTANT_INTEGER: u8 = 3;
const CONSTANT_FLOAT: u8 = 4;
const CONSTANT_LONG: u8 = 5;
const CONSTANT_DOUBLE: u8 = 6;
const CONSTANT_NAME_AND_TYPE: u8 = 12;
const CONSTANT_UTF8: u8 = 1;
const CONSTANT_METHOD_HANDLE: u8 = 15;
const CONSTANT_METHOD_TYPE: u8 = 16;
const CONSTANT_INVOKE_DYNAMI: u8 = 18;

pub enum ConstantInfo {
    Integer(i32)
}

type ConstantPool = Vec<ConstantInfo>;

#[derive(Debug)]
pub struct VersionInfo {
    major_version: u16,
    minor_version: u16,
}

pub trait ClassReader {
    fn read_u8(&self) -> (u8, &[u8]);
    fn read_u16(&self) -> (u16, &[u8]);
    fn read_u32(&self) -> (u32, &[u8]);
    fn read_and_check_magic(&self) -> (u32, &[u8]);
    fn read_and_check_version(&self) -> (VersionInfo, &[u8]);
    fn read_constant_info(&self, constant_pool: &ConstantPool) -> (ConstantInfo, &[u8]);
    fn read_constant_pool(&self) -> (ConstantPool, &[u8]);
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
            // todo
            CONSTANT_INTEGER => {
                let (val, rest) = after_tag.read_u32();
                (ConstantInfo::Integer(val as i32), rest)
            }
            _ => {
                panic!("Wrong tag type");
            }
        }
    }

    fn read_constant_pool(&self) -> (ConstantPool, &[u8]) {
        let (count, after_count) = self.read_u16();
        let mut constant_pool: Vec<ConstantInfo> = Vec::with_capacity(count as usize);

        let mut i: usize = 1;
        let mut rest: &[u8] = after_count;

        while i < (count as usize) {
            let (constant_info, next_rest) = rest.read_constant_info(&constant_pool);
            rest = next_rest;
            match constant_info {
                // todo
                _ => { i = i + 1; }
            }
            constant_pool[i] = constant_info;
        }

        (constant_pool, rest)
    }
}



