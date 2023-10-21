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

use crate::{
    modes::{
        do_asm_get_rip, lambda_moved, lambda_ref, print_dl_addr_info, ptr_meta_run, read_main_ptr,
    },
    support::{
        async_vec,
        constants::{DEBUG_ENABLED, CODE_GEN_ENABLED},
        get_command_line_arguments, CmdArg,
    },
};

use std::env;

pub fn main() -> Result<(), String> {
    let env_args = env::args().collect();
    let args = get_command_line_arguments(&env_args)?;
    let mut is_gdb_mode = false;
    let mut exec_vec = vec![];
    let mut args = args.iter();
    let mut code_gen_opt = None;
    loop {
        let arg = args.next();
        let Some(&arg) = arg else {
            break;
        };
        match arg {
            CmdArg::LongOpt("run") | CmdArg::ShortOpt("r") => {
                let arg2 = *args
                    .next()
                    .ok_or_else(|| format!("Missing option for `{}`", arg))?;
                if let CmdArg::Seq(arg2) = arg2 {
                    exec_vec.push(arg2);
                } else {
                    return Err(format!("Unknown option `{}` for `{}`", arg2, arg));
                }
            }
            CmdArg::LongOpt(value) => match value {
                "gdb" => is_gdb_mode = true,
                "code-gen" => code_gen_opt = Some(true),
                "no-code-gen" => code_gen_opt = Some(false),
                "debug" => unsafe { DEBUG_ENABLED = true },
                "no-debug" => unsafe { DEBUG_ENABLED = false },
                _ => return Err(format!("Invalid option `{}`", arg)),
            },
            _ => {
                return Err(format!("Unknown option `{}`", arg));
            }
        }
    }
    if let Some(code_gen_opt) = code_gen_opt {
        unsafe { CODE_GEN_ENABLED = code_gen_opt }
    }
    for func_name in exec_vec {
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
    if is_gdb_mode {
        loop {}
    }
    Ok(())
}
