use std::fmt;
use std::ptr::drop_in_place;

pub struct TypeName<'a>(pub &'a str);

pub fn forget_namespace(value: &str) -> String {
    value
        .split(' ')
        .map(|x| x.split("::").last().unwrap())
        .collect::<Vec<_>>()
        .join(" ")
}

impl TypeName<'_> {
    pub fn new<T: ?Sized>() -> Self {
        Self(std::any::type_name::<T>())
    }
    pub fn new_v<T: ?Sized>(_: &T) -> Self {
        Self(std::any::type_name::<T>())
    }
    pub fn forget_namespace(&self) -> String {
        forget_namespace(self.0)
    }
}

pub trait NewTypeName {
    fn new_type_name(&self) -> TypeName {
        TypeName::new_v(self)
    }
}

impl<T> NewTypeName for T {}

impl fmt::Display for TypeName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for TypeName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self.0)
    }
}

pub fn get_drop_in_place_typename<T: 'static>(_: &T) -> String {
    drop_in_place::<T>.new_type_name().forget_namespace()
}
