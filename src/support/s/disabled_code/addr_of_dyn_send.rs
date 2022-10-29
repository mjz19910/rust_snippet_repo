use crate::support::describe::describe_ptr_type_parts;
use crate::support::drop_helpers::get_drop_in_place_for;
use crate::support::typename::get_drop_in_place_typename;

pub fn addr_of_dyn_send() {
    let data = [0u64; 8];
    type Ty = &'static dyn Send;
    let ptr = std::ptr::addr_of!(data) as *const () as *const Ty;
    let ptr = unsafe { *ptr };
    describe_ptr_type_parts::<dyn std::any::Any, Ty>();
    println!(
        "{} {:?}",
        get_drop_in_place_typename(&ptr),
        get_drop_in_place_for::<Ty>()
    );
}
