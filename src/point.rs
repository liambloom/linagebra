use core::{ops::{Add, Sub, Mul}, cmp::PartialEq};
use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Add for Point<T>
    where T: Add<Output = T>
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Point<T>
    where T: Sub<Output = T>
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T, U> Mul<U> for Point<T>
    where T: Mul<U, Output = T>,
          U: Copy 
{
    type Output = Self;

    fn mul(self, other: U) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    } 
}
impl<T> Display for Point<T>
    where T: Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}