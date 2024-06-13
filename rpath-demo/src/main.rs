use rpath_sys::hello;
use std::ffi::CStr;

fn main() {
    let hi = unsafe {
        CStr::from_ptr(hello())
    };
    println!("{:?}", hi);
}
