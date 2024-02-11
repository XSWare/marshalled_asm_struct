use std::{
    io::{Read, Write},
    slice,
    str::from_utf8_unchecked,
    time::Instant,
};

#[link(name = "utils")]
extern "C" {
    fn count_character_asm(s: *const u8, len: usize, symbol: u8) -> RawArray;
}

/// return a raw pointer to allocated memory of the requested size.
/// caller is responsible for freeing it.
#[no_mangle]
pub extern "C" fn rust_malloc(len: usize) -> *const u64 {
    let arr = vec![0_u64; len].into_boxed_slice();
    let ptr = Box::leak(arr);
    ptr.as_ptr()
}

#[repr(C)]
struct RawArray {
    ptr: *mut u64,
    len: usize,
}

fn main() {
    let request = "GET / HTTP/1.0\r\n\r\n";
    let mut conn = std::net::TcpStream::connect("google.com:80").unwrap();
    conn.write_all(request.as_bytes()).unwrap();

    let mut buffer = Vec::new();
    conn.read_to_end(&mut buffer).unwrap();

    let html = unsafe { from_utf8_unchecked(&buffer) };
    let search_char = 'a';

    let start = Instant::now();
    let result = count_character(html, search_char);
    let end = Instant::now();

    println!(
        "found {} occurrences of '{}' in string\n'{}'\nat these offsets:\n{:?}\nsearch duration: {}ns",
        result.len(),
        search_char,
        html,
        result,
        (end - start).as_nanos(),
    );
}

fn count_character(s: &str, symbol: char) -> Box<[u64]> {
    unsafe {
        let res = count_character_asm(s.as_ptr(), s.len(), symbol as u8);

        // wrap the raw array inside of a box to free the previously leaked memory after it is dropped.
        let slice = slice::from_raw_parts_mut(res.ptr, res.len);
        Box::from_raw(slice)
    }
}
