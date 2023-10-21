use crate::{
    main,
    support::{get_debug_flag_state, symbol_info::symbol_info_from_addr},
};

pub fn print_dl_addr_info() {
    let info = symbol_info_from_addr(&main);
    let info = if let Some(info) = info {
        info
    } else {
        eprintln!("No symbol info for address");
        return;
    };
    assert!(info.dli_sname.is_none());
    assert!(info.dli_saddr.is_none());
    if get_debug_flag_state() {
        println!("info: {:x?}", info);
    }
    println!(
        "[{}] main @ {:#x?}",
        info.dli_fname.unwrap(),
        main as usize - info.dli_fbase.unwrap() as usize,
    );
}
