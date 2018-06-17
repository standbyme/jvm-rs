mod classfile;

use std::fs::File;
use std::io::prelude::*;
use classfile::ClassReader;

fn main() {
    let path: &str = "Object.class";
    let input: File = File::open(path).unwrap();
    let bytes: Vec<u8> = input.bytes().map(|x| x.unwrap()).collect();
    let (_, after_magic) = bytes.read_and_check_magic();
    println!("{:?}", after_magic);
}
