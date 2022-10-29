use std::{any::Any, ptr::addr_of};

use crate::support::typename::TypeName;

fn log_typename_async_unit() {
    let value = async {};
    println!("{}", TypeName::new_v(&value));
}

fn println_typename_async_unit_ptr_any_meta() {
    let value = async {};
    let parts = (addr_of!(value) as *const dyn Any).to_raw_parts();
    println!("{}", TypeName::new_v(&parts.1));
}
