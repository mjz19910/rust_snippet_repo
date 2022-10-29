use std::any::Any;

pub fn forget_and_no_inline_drop<T>(mut value: T) {
    let ptr = &mut value as *mut T;
    std::mem::forget(value);
    let drop_ptr = get_drop_in_place_for::<T>();
    unsafe { drop_ptr(ptr) };
}

pub fn forget_and_drop<T>(mut value: T) {
    let to_drop = &mut value as *mut T;
    std::mem::forget(value);
    unsafe { std::ptr::drop_in_place(to_drop) };
}

pub fn get_drop_in_place_for<T>() -> unsafe fn(*mut T) {
    std::ptr::drop_in_place::<T>
}

pub fn get_drop_in_place_for_v<T>(_: &T) -> unsafe fn(*mut T) {
    std::ptr::drop_in_place::<T>
}

pub fn get_boxed_drop_in_place_for<'a, T: 'a>() -> Box<(dyn Any + 'a)> {
    Box::new(std::ptr::drop_in_place::<T>)
}
