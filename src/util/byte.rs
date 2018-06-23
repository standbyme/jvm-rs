extern crate byteorder;

use self::byteorder::{ByteOrder, BigEndian};

pub fn i32_to_u8seq(val: i32) -> [u8; 4] {
    let mut buf = [0; 4];
    BigEndian::write_i32(&mut buf, val);
    buf
}

pub fn u8seq_to_i32(val: [u8; 4]) -> i32 {
    BigEndian::read_i32(&val)
}