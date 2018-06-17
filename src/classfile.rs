pub trait ClassReader {
    fn read_uint8(&self) -> (u8, &[u8]);
}

impl ClassReader for [u8] {
    fn read_uint8(&self) -> (u8, &[u8]) {
        let (a, b) = self.split_at(1);
        (a[0], b)
    }
}