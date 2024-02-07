use std::str::from_utf8_unchecked;

#[link(name = "add")]
extern "C" {
    fn asm_add(a: u64, b: u64) -> u64;
}

#[no_mangle]
pub extern "C" fn rust_print(msg_ptr: *const u8, len: usize, val: i64) {
    let message = unsafe { from_utf8_unchecked(std::slice::from_raw_parts(msg_ptr, len)) };
    println!("pointer: {}\nmessage: {}\nlen: {}\nval: {}", msg_ptr as usize, message, len, val);
}

fn main() {
    unsafe { asm_add(1, 2) };
}
