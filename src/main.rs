mod classfile;

use std::fs::File;
use std::io::prelude::*;
use classfile::class_reader::ClassReader;

fn main() {
    let path: &str = "Object.class";
    let input: File = File::open(path).unwrap();
    let bytes: Vec<u8> = input.bytes().map(|x| x.unwrap()).collect();
    let (_, after_magic) = bytes.read_and_check_magic();
    let (version_info, _) = after_magic.read_and_check_version();
    println!("{:?}", version_info);
}
