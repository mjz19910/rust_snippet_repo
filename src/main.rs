#![deny(unsafe_op_in_unsafe_fn)]
#![allow(incomplete_features)]
#![feature(unsize)]
#![feature(ptr_metadata)]
#![feature(generic_const_exprs)]
#![feature(once_cell)]
#![feature(extern_types)]
#![feature(let_chains)]

pub mod exec_mode;
pub mod modes;
pub mod support;

use exec_mode::exec_mode;

pub fn main() -> Result<(), String> {
    exec_mode(main)?;
    Ok(())
}
