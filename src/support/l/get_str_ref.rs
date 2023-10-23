use super::get_type;

pub fn get_str_ref(fns_arr: *const *const ()) -> (*const u8, usize) {
    get_type(fns_arr)
}
