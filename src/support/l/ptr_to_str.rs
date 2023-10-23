use std::ffi::CStr;

pub fn ptr_to_str(ptr: *const i8) -> Option<&'static str> {
    if !ptr.is_null() {
        unsafe { CStr::from_ptr(ptr) }.to_str().ok()
    } else {
        None
    }
}
