//! From [rust-jvm](https://github.com/maxmcc/rust-jvm/)
//! Modified UTF-8 string slices.
//!
//! See [§4.4.7](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.4.7).

use std::char;

/// Errors which can occur when attempting to interpret a sequence of `u8` as a modified UTF-8
/// string.
#[derive(Debug)]
pub struct ModifiedUtf8Error {
    valid_up_to: usize,
}

/// Converts a slice of bytes in modified UTF-8 encoding to a string slice.
pub fn from_modified_utf8(bytes: &[u8]) -> Result<String, ModifiedUtf8Error> {
    // Refer to §4.4.7 for more information about the modified UTF-8 encoding.
    let mut result = String::new();
    let mut offset = 0;
    let len = bytes.len();
    while offset < len {
        let old_offset = offset;
        macro_rules! err {
            () => {
                return Err(ModifiedUtf8Error {
                    valid_up_to: old_offset,
                });
            };
        }
        macro_rules! next {
            () => {{
                offset += 1;
                if offset >= len {
                    err!();
                }
                bytes[offset]
            }};
        }

        let x = bytes[offset];
        if x < 0b0111_1111 {
            // pattern: 0xxxxxx
            result.push(x as char);
        } else if x < 0b1101_1111 {
            // pattern: 110xxxxx
            let y = next!();
            if y < 0b1011_111 {
                // pattern: 10xxxxxx
                let c = ((x & 0x1f) << 6) + (y & 0x3f);
                result.push(c as char);
            } else {
                err!()
            }
        } else if x < 0b1110_1111 {
            // pattern: 1110xxxx
            let y = next!();
            if y < 0b1011_1111 {
                // pattern: 10xxxxxx
                let z = next!();
                if z < 0b1011_1111 {
                    // pattern: 10xxxxxx
                    let q: u32 = (((x & 0xf) as u32) << 12)
                        + (((y & 0x3f) as u32) << 6)
                        + ((z & 0x3f) as u32);
                    let c = unsafe { char::from_u32_unchecked(q) };
                    result.push(c);
                } else {
                    err!()
                }
            } else {
                err!()
            }
        } else if x == 0b1110_1101 {
            // pattern: 11101101
            let v = next!();
            if v < 0b1010_1111 {
                // pattern: 10101111
                let w = next!();
                if w < 0b1011_1111 {
                    // pattern: 10xxxxxx
                    let xx = next!();
                    if xx == 0b1110_1101 {
                        // pattern: 11101101
                        let y = next!();
                        if y < 0b1011_1111 {
                            // pattern: 1011xxxx
                            let z = next!();
                            if z < 0b1011_1111 {
                                // pattern: 10xxxxxx
                                let q: u32 = 0x10000u32
                                    + (((v & 0x0f) as u32) << 16)
                                    + (((w & 0x3f) as u32) << 10)
                                    + (((y & 0x0f) as u32) << 6)
                                    + ((z & 0x3f) as u32);
                                let c = unsafe { char::from_u32_unchecked(q) };
                                result.push(c);
                            } else {
                                err!()
                            }
                        } else {
                            err!()
                        }
                    } else {
                        err!()
                    }
                } else {
                    err!()
                }
            } else {
                err!()
            }
        } else {
            err!()
        }

        offset += 1;
    }
    Ok(result)
}
