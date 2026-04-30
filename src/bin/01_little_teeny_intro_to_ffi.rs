use libc::{c_int, c_ulong};

#[link(name = "z")] // this basically says we need to link this to the zlib
unsafe extern "C" {
    fn compress(
        dest: *mut u8,
        dest_len: *mut c_ulong,
        source: *const u8,
        source_len: c_ulong,
    ) -> c_int;

    fn compressBound(source_len: c_ulong) -> c_ulong;

    fn uncompress(
        dest: *mut u8,
        dest_len: *mut c_ulong,
        source: *const u8,
        source_len: c_ulong,
    ) -> c_int;
}

pub fn zlib_compress(source: &[u8]) -> Vec<u8> {
    unsafe {
        let source_len = source.len() as c_ulong;

        let mut dest_len = compressBound(source_len);
        let mut dest = Vec::with_capacity(dest_len as usize);

        compress(
            dest.as_mut_ptr(),
            &mut dest_len,
            source.as_ptr(),
            source_len,
        );

        dest.set_len(dest_len as usize);
        dest
    }
}

fn main() {
    let hello_world = "Hello World".as_bytes();
    let hello_world_compressed = zlib_compress(&hello_world);

    println!("Raw Bytes: {hello_world:?}");
    println!("Compressed: {hello_world_compressed:?}");
}
