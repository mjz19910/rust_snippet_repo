use crate::support::symbol_info::symbol_info_from_addr;

pub fn print_dl_addr_info<T>(main_func: fn() -> T) {
    let info = symbol_info_from_addr(&main_func);
    let info = if let Some(info) = info {
        info
    } else {
        eprintln!("No symbol info for address");
        return;
    };
    assert!(info.dli_sname.is_none());
    assert!(info.dli_saddr.is_none());
    if cfg!(feature = "debug") {
        println!("info: {:x?}", info);
    }
    println!(
        "[{}] main @ {:#x?}",
        info.dli_fname.unwrap(),
        main_func as usize - info.dli_fbase.unwrap() as usize,
    );
}
