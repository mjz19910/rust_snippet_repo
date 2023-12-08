#![deny(unsafe_op_in_unsafe_fn)]
#![allow(incomplete_features)]
#![feature(unsize)]
#![feature(ptr_metadata)]
#![feature(generic_const_exprs)]
#![feature(once_cell)]
#![feature(extern_types)]
#![feature(let_chains)]

pub mod modes;
pub mod run_one;
pub mod support;

use run_one::run_one;

use std::time::Duration;

use support::{constants::DEBUG_ENABLED, get_command_line_arguments, ArgParser};

pub fn main() -> Result<(), String> {
    let args = get_command_line_arguments()?;
    let parsed_args = ArgParser { args }.parse_args()?;
    unsafe {
        DEBUG_ENABLED = parsed_args.debug_enabled;
    }
    for func_name in parsed_args.run_options {
        run_one(func_name.as_str())?;
    }
    if parsed_args.gdb_delay_loop {
        loop {
            std::thread::sleep(Duration::from_nanos(1_000));
        }
    }
    Ok(())
}
