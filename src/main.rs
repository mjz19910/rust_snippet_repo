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

use support::ArgParser;

use crate::{
    modes::{
        do_asm_get_rip, lambda_moved, lambda_ref, print_dl_addr_info, ptr_meta_run, read_main_ptr,
    },
    support::{
        async_vec,
        constants::{CODE_GEN_ENABLED, DEBUG_ENABLED},
        get_command_line_arguments,
    },
};

use std::env;

pub fn main() -> Result<(), String> {
    let env_args = env::args().collect();
    let args = get_command_line_arguments(&env_args)?;
    let parsed_args = ArgParser { args }.parse_args()?;
    unsafe {
        CODE_GEN_ENABLED = parsed_args.code_gen_enabled;
        DEBUG_ENABLED = parsed_args.debug_enabled;
    }
    for func_name in parsed_args.run_options {
        match func_name {
            "async_vec" => async_vec(),

            // For modes modules
            "do_asm_get_rip" => do_asm_get_rip(),
            "lambda_ref" => lambda_ref(),
            "lambda_moved" => lambda_moved(),
            "print_dl_addr_info" => print_dl_addr_info(),
            "ptr_meta_run" => ptr_meta_run()?,
            "read_main_ptr" => read_main_ptr(),

            // None
            "none" => (),

            // Error
            _ => return Err(format!("Unknown function `{}`", func_name)),
        }
    }
    if parsed_args.gdb_delay_loop {
        loop {}
    }
    Ok(())
}
