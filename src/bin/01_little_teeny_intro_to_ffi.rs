use libc::{c_int, c_ulong};

#[link(name = "z")] // this basically says we need to link this to the zlib
unsafe extern "C" {
    fn compress(
        dest: *mut u8,
        dest_len: *mut c_ulong,
        source: *const u8,
        source_len: *mut c_ulong,
    ) -> c_int;
    
    fn compressBound(source_len: c_ulong) -> c_ulong;
    
    fn uncompress(
        dest: *mut u8,
        dest_len: *mut c_ulong,
        source: *const u8,
        source_len: *mut c_ulong,
    ) -> c_int;
}

fn main() {
    
}