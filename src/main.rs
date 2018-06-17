use std::fs::File;
use std::io::prelude::*;

mod classfile;

use classfile::ClassReader;

fn main() {
    let path: &str = "Object.class";
    let input: File = File::open(path).unwrap();
    let bytes: Vec<u8> = input.bytes().map(|x| x.unwrap()).collect();
    let (a, b) = bytes.read_uint8();
    println!("{:?}", a);
    let (c, d) = b.read_uint8();
    println!("{:?}",c);
    println!("{:?}",d);

}
