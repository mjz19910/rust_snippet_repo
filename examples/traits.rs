use std::fmt::{Debug, Display};

trait NewTrait: Debug + Display {}

impl NewTrait for i32 {}

fn main() {
    let a: (i32, Box<dyn NewTrait>) = (1, Box::new(2));
    println!("{}, {}", a.0, a.1);
}
