
extern crate libc;

use std::{str, slice};
use libc::size_t;




fn print_byte_slice_as_utf8(bytes: &[u8]) {
    match str::from_utf8(bytes) {
        Ok(s) => println!("{}", s),
        Err(_) => println!("Invalid utf8"),
    }
}
#[no_mangle]
pub extern "C" fn uft8_bytes_to_rust(bytes: *const u8, len: size_t) {
    let byte_slice = unsafe { slice::from_raw_parts(bytes, len as usize) };
    print_byte_slice_as_utf8(byte_slice);

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
