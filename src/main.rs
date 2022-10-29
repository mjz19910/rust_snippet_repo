#![deny(unsafe_op_in_unsafe_fn)]
#![allow(incomplete_features)]
#![feature(unsize)]
#![feature(ptr_metadata)]
#![feature(generic_const_exprs)]
#![feature(once_cell)]
#![feature(extern_types)]
#![feature(let_chains)]
pub mod exec_mode;
/// #![feature(once_cell)] for `use std::cell::LazyCell;`
/// #![feature(generic_const_exprs)] for `let x:Type<{ N + {number}}>`
/// #![allow(incomplete_features)] for #![feature(generic_const_exprs)]
/// #![feature(ptr_metadata)] for `use std::ptr::DynMetadata;`
/// #![feature(unsize)] for `use std::marker::Unsize;`
/// #![feature(let_chains)] for `if let .. = .. && ..`
pub mod modes;
pub mod support;
use exec_mode::exec_mode;

pub fn main() -> Result<(), String> {
    exec_mode(main)?;
    Ok(())
}
