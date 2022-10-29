use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct RawVecInternal<T> {
    pub ptr: NonNull<T>,
    pub cap: usize,
    pub alloc: *const (),
    pub phantom: PhantomData<T>,
}

pub struct VecInternal<T> {
    pub buf: RawVecInternal<T>,
    pub len: usize,
}
