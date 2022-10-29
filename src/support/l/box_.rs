use std::fmt;

pub struct AVec<T> {
    value: Vec<T>,
}

impl<T> AVec<T> {
    pub fn new(vec: Vec<T>) -> Self {
        Self { value: vec }
    }
}

impl<T: fmt::Display> fmt::Display for AVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "[")?;

        for value in &self.value[0..self.value.len() - 1] {
            write!(f, "{}", value)?;
            write!(f, ", ")?;
        }

        write!(f, "{}", self.value[self.value.len() - 1])?;
        write!(f, "]")
    }
}
