use std::fmt::{Debug, Display};

trait NewTrait: Debug + Display {
    type Output;
    fn get(&self) -> Self::Output;
}

impl<T> NewTrait for T
where
    T: Debug + Display,
{
    type Output = T;
    fn get(&self) -> T {
        let v = self as *const T;
        unsafe { std::ptr::read(v) }
    }
}
fn main() {
    let a: (i32, Box<dyn NewTrait<Output = _>>) = (1, Box::new(2));
    let v = (*a.1).get();
    println!("{}, {:?}", a.0, v);
}
