use std::mem::MaybeUninit;

#[derive(Debug)]
#[repr(C)]
pub(crate) struct RawDLInfo {
    pub dli_fname: *const i8,
    pub dli_fbase: *const (),
    pub dli_sname: *const i8,
    pub dli_saddr: *const (),
}

extern "C" {
    pub(crate) fn dladdr(addr: *const (), info: *mut RawDLInfo) -> u32;
}

pub(crate) fn raw_symbol_info_from_addr<T: ?Sized>(ptr: *const T) -> Option<RawDLInfo> {
    let mut uninit_info = MaybeUninit::zeroed();
    let res = unsafe { dladdr(ptr.cast(), uninit_info.as_mut_ptr()) };
    let info = unsafe { uninit_info.assume_init() };
    if res != 0 {
        return Some(info);
    }
    None
}
