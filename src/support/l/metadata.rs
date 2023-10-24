use std::{
    fmt::{self, Debug},
    marker::{PhantomData, Unsize},
    ptr::{DynMetadata, Pointee},
};

use super::{get_debug_flag_state, PtrIter};

pub const fn new_metadata<Dyn: ?Sized, T>() -> DynMetadata<Dyn>
where
    T: Unsize<Dyn>,
    Dyn: Pointee<Metadata = DynMetadata<Dyn>>,
{
    (std::ptr::null::<T>() as *const Dyn).to_raw_parts().1
}

pub const fn get_metadata<Dyn: ?Sized, T>(value: *const T) -> DynMetadata<Dyn>
where
    T: Unsize<Dyn>,
    Dyn: Pointee<Metadata = DynMetadata<Dyn>>,
{
    (value as *const Dyn).to_raw_parts().1
}

pub trait GetX {
    fn x_value(&self) -> isize;
}
impl dyn GetX {
    pub fn default_box() -> Box<Self> {
        Box::new((|| -1) as fn() -> _)
    }
}

impl<T, const X: usize> GetX for XVTable<T, X> {
    fn x_value(&self) -> isize {
        X as isize
    }
}

impl GetX for fn() -> isize {
    fn x_value(&self) -> isize {
        self()
    }
}

#[repr(C)]
pub struct XVTable<T: ?Sized, const X: usize> {
    pub drop_in_place: unsafe fn(*mut T),
    pub size_of: usize,
    pub align_of: usize,
    pub vtable_fns: [*const (); X],
    pub phantom: PhantomData<T>,
}
impl<T: ?Sized, const X: usize> XVTable<T, X> {
    pub fn debug(&self, state: &PtrIter, name: &str) {
        if !get_debug_flag_state() {
            return;
        }
        println!(
            "{} p_dbg_ptr: {}({}, {:#x})",
            state.p_dbg(),
            name,
            X,
            state.ptr_base,
        );
        println!("{} p_dbg_vtb: {:x?}", state.p_dbg(), self);
    }
}

impl<T: ?Sized, const X: usize> Debug for XVTable<T, X> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("VTable")?;
        write!(f, "<{}>", X)?;
        f.write_str(" { ")?;
        write!(f, "drop_in_place: {:?}, ", self.drop_in_place)?;
        write!(f, "size_of: 0x{:02x?}, ", self.size_of)?;
        write!(f, "align_of: 0x{:02x?}, ", self.align_of)?;
        f.write_str("vtable_fns: [")?;
        if !self.vtable_fns.is_empty() {
            write!(f, "{:?}", self.vtable_fns[0])?;
        }
        for val in self.vtable_fns.iter().skip(1).take(1) {
            write!(f, ", {:?}", val)?;
        }
        if self.vtable_fns.len() > 2 {
            f.write_str(", ..] }")
        } else {
            f.write_str("] }")
        }
    }
}

impl<'a, T: ?Sized, const N: usize> XDynMetadata<'a, T, N> {
    pub const fn vtable(&self) -> &'a XVTable<T, N> {
        self.vtable_ptr
    }
    pub const fn vtable_copy(&self) -> XVTable<T, N> {
        *self.vtable_ptr
    }
}

pub struct XDynMetadata<'a, T: ?Sized, const X: usize> {
    pub vtable_ptr: &'a XVTable<T, X>,
    phantom: std::marker::PhantomData<T>,
}

impl<T: ?Sized, const X: usize> fmt::Debug for XDynMetadata<'_, T, X> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DynMetadata")
            .field(&self.vtable_ptr)
            .finish()
    }
}

impl<T> Copy for XDynMetadata<'_, T, 1> {}
impl<T> Clone for XDynMetadata<'_, T, 1> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized, const X: usize> Copy for XVTable<T, X> {}
impl<T: ?Sized, const X: usize> Clone for XVTable<T, X> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

pub const fn get_vtable<'a, T: ?Sized, const X: usize>(meta: &'a DynMetadata<T>) -> &XVTable<T, X> {
    get_metadata_ext(meta).vtable()
}

pub const fn get_vtable_v<T: ?Sized>(meta: DynMetadata<T>) -> XVTable<T, 1> {
    get_metadata_ext_v(meta).vtable_copy()
}

pub const fn get_metadata_ext<'a, T: ?Sized, const X: usize>(
    meta: &DynMetadata<T>,
) -> &XDynMetadata<'a, T, X> {
    unsafe { std::mem::transmute(meta) }
}

pub const fn get_metadata_ext_v<'a, T: ?Sized, const X: usize>(
    meta: DynMetadata<T>,
) -> XDynMetadata<'a, T, X> {
    unsafe { std::mem::transmute(meta) }
}
