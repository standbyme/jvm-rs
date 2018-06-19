extern crate vec_map;

mod classfile;
mod util;

use std::fs::File;
use std::io::prelude::*;
use classfile::class_reader::ClassReader;

fn main() {
    let path: &str = "Object.class";
    let input: File = File::open(path).unwrap();
    let bytes: Vec<u8> = input.bytes().map(|x| x.unwrap()).collect();
    let class_file = bytes.parse();
    println!("{:?}", class_file);
}
