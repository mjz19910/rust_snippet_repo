use std::{
    fmt::{Debug, Display},
    mem::ManuallyDrop,
};

trait NewTrait: Debug + Display {
    type Output;
    fn get(&self) -> ManuallyDrop<Self::Output>;
}

impl<T> NewTrait for T
where
    T: Debug + Display,
{
    type Output = T;
    fn get(&self) -> ManuallyDrop<T> {
        let v = self as *const T;
        ManuallyDrop::new(unsafe { std::ptr::read(v) })
    }
}
fn main() {
    let a: (i32, Box<dyn NewTrait<Output = _>>) = (1, Box::new(2));
    let v = a.1.get();
    println!("{}, {:?}", a.0, v.as_ref());
}
