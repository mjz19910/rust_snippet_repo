use super::{get_type, RawLocation};

pub fn get_location(fns_arr: *const *const ()) -> RawLocation {
    get_type(fns_arr)
}
