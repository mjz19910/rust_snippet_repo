use std::any::Any;
use std::fmt::Debug;
use std::ptr::addr_of;

use crate::support::describe::describe_drop_in_place_for;
use crate::support::describe::describe_ptr_parts;
use crate::support::describe::describe_ptr_type_parts;

pub fn describe_any() {
    describe_ptr_type_parts::<dyn Any, u8>();
    describe_ptr_type_parts::<dyn Any, u16>();
}
pub fn describe_any_ref() {
    describe_ptr_type_parts::<dyn Any, &dyn Send>();
}
pub fn describe_u8() {
    describe_ptr_type_parts::<dyn Send, u8>();
    describe_ptr_type_parts::<dyn Sync, u8>();
    describe_ptr_type_parts::<dyn Debug, u8>();
}
pub fn describe_box_send() {
    describe_ptr_type_parts::<dyn Send, Box<dyn Send>>();
}
pub fn describe_sync_ref() {
    describe_ptr_type_parts::<dyn Sync, &dyn Sync>();
}
fn describe_any_vec_u64() {
    describe_ptr_type_parts::<dyn Any, Vec<u64>>();
    describe_drop_in_place_for::<Vec<u64>>();
}

fn describe_async_unit_type() {
    use std::future::Future;
    let value = async {};
    let v_ref = &value as &dyn Future<Output = _>;
    let ptr = v_ref as *const dyn Future<Output = _>;
    let parts = ptr.to_raw_parts();
    use crate::support::typename::TypeName;
    println!("{}", TypeName::new_v(v_ref));
    println!("{}", TypeName::new_v(&parts.1));
    println!("{:?}", parts);
    describe_ptr_parts(&parts);
}

fn describe_async_unit_as_any() {
    let value = async {};
    let parts = (addr_of!(value) as *const dyn Any).to_raw_parts();
    describe_ptr_parts(&parts);
}
