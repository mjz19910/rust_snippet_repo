pub fn add<T>(a: &mut *const T, n: usize) {
    *a = unsafe { a.add(n) };
}

pub fn sub<T>(a: &mut *const T, n: usize) {
    *a = unsafe { a.sub(n) };
}
