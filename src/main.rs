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
    println!("major_version {:?}", class_file.major_version);
    println!("minor_version {:?}", class_file.minor_version);
    println!("constant_pool {:?}", class_file.constant_pool);
    println!("access_flags {:?}", class_file.access_flags);
    println!("this_class {:?}", class_file.this_class);
    println!("super_class {:?}", class_file.super_class);
    println!("interfaces {:?}", class_file.interfaces);
    println!("fields {:?}", class_file.fields);
    println!("methods_count {:?}", class_file.methods.len());
    println!("methods {:?}", class_file.methods);

    println!("attributes {:?}", class_file.attributes);
}
