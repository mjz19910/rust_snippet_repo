use crate::{
    modes::{
        do_asm_get_rip::do_asm_get_rip, lambda_ref::lambda_ref,
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

use crate::modes::lambda_moved::lambda_moved;

use std::env;

pub fn exec_mode<U>(main_addr: fn() -> U) -> Result<(), String> {
    let env_args = env::args().collect();
    let args = get_command_line_arguments(&env_args)?;
    let mut is_gdb_mode = false;
    let mut exec_vec = vec![];
    let mut capture_next_arg_to_exec = false;
    for arg in args {
        match arg {
            CmdArg::LongOpt(value) => match value {
                // --gdb
                "gdb" => is_gdb_mode = true,
                // --code-gen
                "code-gen" => unsafe { FORCE_CODE_GEN = true },
                // --no-code-gen
                "no-code-gen" => unsafe { SKIP_CODE_GEN = true },
                // --run-meta
                "run" => capture_next_arg_to_exec = true,
                // --debug
                "debug" => unsafe { FORCE_DEBUG_FLAG = true },
                // --no-debug
                "no-debug" => unsafe { SKIP_DEBUG_FLAG = true },
                _ => return Err(format!("Invalid option '--{}'", value)),
            },
            CmdArg::ShortOpt(value) => match value {
                // -r
                "r" => capture_next_arg_to_exec = true,
                _ => return Err(format!("Invalid option '-{}'", value)),
            },
            CmdArg::Seq(value) if capture_next_arg_to_exec => exec_vec.push(value),
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
            "print_dl_addr_info" => print_dl_addr_info(main_addr),
            "ptr_meta_run" => ptr_meta_run(main_addr as *const u8)?,
            "read_main_ptr" => read_main_ptr(main_addr),

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
