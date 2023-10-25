use std::fmt::{Debug, Display};

trait NewTrait: Debug + Display {
    type Output;
    fn get(&self) -> Self::Output;
}

impl NewTrait for i32 {
    type Output = i32;
    fn get(&self) -> i32 {
        *self
    }
}
fn main() {
    let a: (i32, Box<dyn NewTrait<Output = _>>) = (1, Box::new(2));
    let v: i32 = a.1.get();
    println!("{}, {}", a.0, v);
}
