use std::ffi::CString;

pub fn sys_tonlib_set_verbosity_level(level: u32) {
    unsafe {
        tonlib_sys::tonlib_client_set_verbosity_level(level);
    }
}
pub fn sys_tonlib_client_set_verbosity_level(level: u32) {
    unsafe {
        tonlib_sys::tonlib_client_set_verbosity_level(level);
    }
}

// ptrs must be valid pointers to c_char
// pub fn sys_cleanup_ptrs<T: AsRef<[*mut std::os::raw::c_char]>>(ptrs: T) {
//     let ptrs_ref = ptrs.as_ref();
//     for ptr in ptrs_ref {
//         if ptr.is_null() { continue; }
//         let _ = unsafe {CString::from_raw(*ptr)};
//     }
// }
