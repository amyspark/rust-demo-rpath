use std::ffi::c_char;

extern "C" {
    #[link_name = "hello"]
    pub fn hello() -> *const c_char;
}
