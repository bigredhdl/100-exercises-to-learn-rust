// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.

#[derive(Debug, Copy)]
pub struct WrappingU32 {
    value: u32,
}

// Implement the `Add` trait for `WrappingU32`
// such that it wraps around on overflow.
use std::ops::Add;
impl Add for WrappingU32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            value: self.value.wrapping_add(rhs.value),
        }
    }
}

// Implement PartialEq for WrappingU32
use std::cmp::PartialEq;
impl PartialEq for WrappingU32 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

// Implement the Clone trait for WrappingU32
impl Clone for WrappingU32 {
    fn clone(&self) -> Self {
        Self { value: self.value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
