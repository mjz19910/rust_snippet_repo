use std::{
    marker::Unsize,
    ptr::{DynMetadata, Pointee},
};

use super::{drop_helpers::get_drop_in_place_for, metadata::get_vtable, typename::TypeName};

pub fn describe_ptr_parts<T: ?Sized>(parts: &(*const (), DynMetadata<T>)) {
    let vtable = get_vtable::<T, 1>(parts.1);
    println!("({:?}, {:?} -> {:?})", parts.0, parts.1, vtable);
}

pub const fn new_ptr_parts<Dyn: ?Sized, T>() -> (*const (), DynMetadata<Dyn>)
where
    T: Unsize<Dyn>,
    Dyn: Pointee<Metadata = DynMetadata<Dyn>>,
{
    (std::ptr::null::<T>() as *const Dyn).to_raw_parts()
}

pub fn describe_ptr_type_parts<Dyn: ?Sized, T>()
where
    T: Unsize<Dyn>,
    Dyn: Pointee<Metadata = DynMetadata<Dyn>>,
{
    print!(
        "<{}, {}> ",
        TypeName::new::<Dyn>().forget_namespace(),
        TypeName::new::<T>().forget_namespace()
    );
    let parts = (std::ptr::null::<T>() as *const Dyn).to_raw_parts();
    describe_ptr_parts(&parts);
    assert_eq!(get_vtable::<Dyn, 1>(parts.1).size_of, parts.1.size_of());
    assert_eq!(get_vtable::<Dyn, 1>(parts.1).align_of, parts.1.align_of());
}

pub fn describe_drop_in_place_for<T>() {
    println!(
        "drop_in_place::<{}> -> {: >66?}",
        TypeName::new::<T>().forget_namespace(),
        get_drop_in_place_for::<T>()
    );
}
