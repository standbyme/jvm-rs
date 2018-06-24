use std;

pub fn i64_to_i32seq(val: i64) -> [i32; 2] {
    unsafe {
        std::mem::transmute::<i64, [i32; 2]>(val)
    }
}

pub fn i32seq_to_i64(val: [i32; 2]) -> i64 {
    unsafe {
        std::mem::transmute::<[i32; 2], i64>(val)
    }
}