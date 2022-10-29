pub fn get_type<T: Copy, U>(fns_arr: *const U) -> T {
    let fns_arr = fns_arr as *const T;
    unsafe { *fns_arr }
}
