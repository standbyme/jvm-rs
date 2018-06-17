use std::fs::File;
use std::io::prelude::*;

fn main() {
    let path: &str = "Object.class";
    let input: File = File::open(path).unwrap();
    let bytes = input.bytes().map(|x| x.unwrap());
    for byte in bytes {
        println!("{:X}", byte);
    }
}
