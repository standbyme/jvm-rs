extern crate byteorder;

use self::byteorder::{ByteOrder, BigEndian};

pub trait ClassReader {
    fn read_u8(&self) -> (u8, &[u8]);
    fn read_u32(&self) -> (u32, &[u8]);
    fn read_and_check_magic(&self) -> (u32, &[u8]);
}

impl ClassReader for [u8] {
    fn read_u8(&self) -> (u8, &[u8]) {
        let (a, b) = self.split_at(1);
        (a[0], b)
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
}