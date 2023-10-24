use std::{
    marker::Unsize,
    ptr::{DynMetadata, Pointee},
};

use super::{
    drop_helpers::get_drop_in_place_for,
    metadata::{get_vtable, XVTable},
    typename::TypeName,
};

pub struct PtrParts<T: ?Sized> {
    data: *const (),
    metadata: DynMetadata<T>,
}
impl<T: ?Sized> PtrParts<T> {
    pub fn get_vtable<const X: usize>(&self) -> &XVTable<T, X> {
        get_vtable::<T, X>(&self.metadata)
    }
    pub fn describe(&self) {
        let vtable = self.get_vtable::<1>();
        println!("({:?}, {:?} -> {:?})", self.data, self.metadata, vtable);
    }
    pub fn from_raw_parts(value: (*const (), DynMetadata<T>)) -> Self {
        Self {
            data: value.0,
            metadata: value.1,
        }
    }
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
    let parts = PtrParts::from_raw_parts((std::ptr::null::<T>() as *const Dyn).to_raw_parts());
    parts.describe();
    assert_eq!(parts.get_vtable::<1>().size_of, parts.metadata.size_of());
    assert_eq!(parts.get_vtable::<1>().align_of, parts.metadata.align_of());
}

pub fn describe_drop_in_place_for<T>() {
    println!(
        "drop_in_place::<{}> -> {: >66?}",
        TypeName::new::<T>().forget_namespace(),
        get_drop_in_place_for::<T>()
    );
}
