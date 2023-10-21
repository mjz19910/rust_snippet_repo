use crate::{
    modes::{
        do_asm_get_rip::do_asm_get_rip, lambda_moved::lambda_moved, lambda_ref::lambda_ref,
        print_dl_addr_info::print_dl_addr_info, ptr_meta_run::ptr_meta_run,
    },
    support::{
        asm::jmp_loop,
        cmd_arg::CmdArg,
        constants::{FORCE_CODE_GEN, FORCE_DEBUG_FLAG, SKIP_CODE_GEN, SKIP_DEBUG_FLAG},
        disabled_code::drop_helpers::drop_helpers_async_vec_i32,
        get_command_line_arguments::get_command_line_arguments,
    },
};

use std::env;

pub fn exec_mode() -> Result<(), String> {
    let env_args = env::args().collect();
    let args = get_command_line_arguments(&env_args)?;
    let mut is_gdb_mode = false;
    let mut exec_vec = vec![];
    let mut args = args.iter();
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
                "code-gen" => unsafe { FORCE_CODE_GEN = true },
                "no-code-gen" => unsafe { SKIP_CODE_GEN = true },
                "debug" => unsafe { FORCE_DEBUG_FLAG = true },
                "no-debug" => unsafe { SKIP_DEBUG_FLAG = true },
                _ => return Err(format!("Invalid option `{}`", arg)),
            },
            _ => {
                return Err(format!("Unknown option `{:?}`", arg));
            }
        }
    }
    for func_name in exec_vec {
        use crate::modes::read_main_ptr::read_main_ptr;
        match func_name {
            "drop_helpers_async_vec_i32" => drop_helpers_async_vec_i32(),

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
        jmp_loop();
    }
    Ok(())
}
