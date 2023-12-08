use crate::{
    modes::{do_asm_get_rip, lambda_moved, lambda_ref, print_dl_addr_info, read_main_ptr},
    support::{async_vec, PtrIter},
};

pub fn run_one(function_name: &str) -> Result<(), String> {
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
            let ref mut x2 = 1;
            println!("{x2:?}");
        }

        // None
        "none" => (),

        // Error
        _ => return Err(format!("Unknown function `{}`", function_name)),
    }
    Ok(())
}
