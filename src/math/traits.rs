pub use std::ops::{Add, Mul, Rem};
pub use std::fmt;
pub use std::convert::*;

pub trait One {
    fn one(&self) -> Self;
}

impl<T: From<u8>> One for T where {
    fn one(&self) -> Self {
        Self::from(1)
    }
}

