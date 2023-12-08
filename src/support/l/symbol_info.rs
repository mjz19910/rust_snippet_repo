use std::ptr;

use super::{
    ptr_to_str,
    symbol_info_ffi::{raw_symbol_info_from_addr, RawDLInfo},
};

#[derive(Copy, Clone, Debug)]
pub struct DLInfo {
    pub dli_fname: Option<&'static str>,
    pub dli_fbase: Option<*const ()>,
    pub dli_sname: Option<&'static str>,
    pub dli_saddr: Option<*const ()>,
}

impl DLInfo {
    fn new(src_info: RawDLInfo) -> DLInfo {
        if src_info.dli_saddr.is_null() {
            return Self {
                dli_fname: unsafe { ptr_to_str(src_info.dli_fname) },
                dli_fbase: Some(src_info.dli_fbase),
                dli_sname: unsafe { ptr_to_str(src_info.dli_sname) },
                dli_saddr: None,
            };
        }
        Self {
            dli_fname: unsafe { ptr_to_str(src_info.dli_fname) },
            dli_fbase: Some(src_info.dli_fbase),
            dli_sname: unsafe { ptr_to_str(src_info.dli_sname) },
            dli_saddr: Some(src_info.dli_saddr),
        }
    }
}

pub fn symbol_info_from_addr<T>(x: &T) -> Option<DLInfo> {
    let x = x as *const T as *const ();
    let info = raw_symbol_info_from_addr(x)?;
    Some(DLInfo::new(info))
}

pub fn symbol_info_from_fn<T, U>(value: fn(T) -> U) -> Option<DLInfo> {
    let fn_ptr = &value as *const fn(T) -> U as *const *const ();
    let ptr = unsafe { *fn_ptr };
    let info = raw_symbol_info_from_addr(ptr)?;
    Some(DLInfo::new(info))
}

pub trait SymbolInfo {
    fn symbol_info(&self) -> Option<DLInfo> {
        let info = raw_symbol_info_from_addr(self)?;
        Some(DLInfo::new(info))
    }
}

impl<T> SymbolInfo for fn() -> T {}
impl<T, U> SymbolInfo for fn(T) -> U {}
impl<T, U> SymbolInfo for unsafe fn(T) -> U {}
impl<T> SymbolInfo for *const T {
    fn symbol_info(&self) -> Option<DLInfo> {
        let info = raw_symbol_info_from_addr(*self)?;
        Some(DLInfo::new(info))
    }
}

/// get the file base from dynamic loader info about the
/// address queried
pub fn get_dli_fbase(info: DLInfo) -> Option<*const ()> {
    info.dli_fbase
}

pub fn symbol_info_and_ptr_from_addr<T: ?Sized>(ptr: &T) -> (*const (), *const ()) {
    let ptr = ptr as *const T as *const ();
    let info = raw_symbol_info_from_addr(ptr);
    match info {
        Some(info) => (info.dli_fbase, ptr),
        None => (ptr::null(), ptr),
    }
}

pub fn symbol_info_and_ptr_from_fn<T, U>(value: fn(T) -> U) -> (*const (), *const ()) {
    let fn_ptr = &value as *const fn(T) -> U as *const *const ();
    let ptr = unsafe { *fn_ptr };
    let info = raw_symbol_info_from_addr(ptr);
    match info {
        Some(info) => (info.dli_fbase, ptr),
        None => (ptr::null(), ptr),
    }
}

pub fn symbol_info_and_ptr_from_fn_0<U>(value: fn() -> U) -> (*const (), *const ()) {
    let fn_ptr = &value as *const fn() -> U as *const *const ();
    let ptr = unsafe { *fn_ptr };
    let info = raw_symbol_info_from_addr(ptr);
    match info {
        Some(info) => (info.dli_fbase, ptr),
        None => (ptr::null(), ptr),
    }
}
