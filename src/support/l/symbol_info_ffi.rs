use std::ffi::CStr;
use std::mem::MaybeUninit;

#[derive(Debug)]
#[repr(C)]
pub(crate) struct DLInfo {
    pub dli_fname: *const i8,
    pub dli_fbase: *const (),
    pub dli_sname: *const i8,
    pub dli_saddr: *const (),
}

extern "C" {
    pub(crate) fn dladdr(addr: *const (), info: *mut DLInfo) -> u32;
}

pub(crate) fn symbol_info_from_addr<T: ?Sized>(ptr: *const T) -> Option<DLInfo> {
    let mut uninit_info = MaybeUninit::zeroed();
    let res = unsafe { dladdr(ptr.cast(), uninit_info.as_mut_ptr()) };
    let info = unsafe { uninit_info.assume_init() };
    if res != 0 {
        return Some(info);
    }
    None
}

pub(crate) fn ptr_to_str(ptr: *const i8) -> Option<&'static str> {
    if !ptr.is_null() {
        unsafe { CStr::from_ptr(ptr) }.to_str().ok()
    } else {
        None
    }
}
