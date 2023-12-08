#![deny(unsafe_op_in_unsafe_fn)]
#![allow(incomplete_features)]
#![feature(unsize)]
#![feature(ptr_metadata)]
#![feature(generic_const_exprs)]
#![feature(once_cell)]
#![feature(extern_types)]
#![feature(let_chains)]

pub mod modes;
pub mod support;

use std::time::Duration;

use support::{
    async_vec, constants::DEBUG_ENABLED, get_command_line_arguments, ArgParser, PtrIter,
};

use crate::modes::{do_asm_get_rip, lambda_moved, lambda_ref, print_dl_addr_info, read_main_ptr};

fn run_one(function_name: &str) -> Result<(), String> {
    match function_name {
        "async_vec" => async_vec(),

        // For modes modules
        "do_asm_get_rip" => do_asm_get_rip(),
        "lambda_ref" => lambda_ref(),
        "lambda_moved" => lambda_moved(),
        "print_dl_addr_info" => print_dl_addr_info(),
        "ptr_iter_run" => PtrIter::new()?.run()?,
        "read_main_ptr" => read_main_ptr(),
        "array_1" => {
            let x = [40, 100];
            println!("{x:?}");
            let x2 = &mut 1;
            println!("{x2:?}");
        }

        // None
        "none" => (),

        // Error
        _ => return Err(format!("Unknown function `{}`", function_name)),
    }
    Ok(())
}

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
