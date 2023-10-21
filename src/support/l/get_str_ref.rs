use super::{get_type, RawStrRef};

pub(crate) fn get_str_ref(fns_arr: *const *const ()) -> RawStrRef {
    get_type(fns_arr)
}
