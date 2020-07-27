use core::{ops::{Add, Sub, Mul, Neg, Index, IndexMut}, cmp::PartialEq};
use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Debug, Clone)]
pub struct Vector<T> {
    v: Vec<T>
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.v[i]
    }
}

impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.v[i]
    }
}

impl<T> Add for Vector<T>
    where T: Add<T, Output = T> + Copy
{
    type Output = Option<Self>;

    fn add(self, o: Self) -> Self::Output {
        if self.len() == o.len() {
            Some(Self {
                v: self.v.iter().enumerate().map(|i|  *i.1 + o[i.0]).collect(),
            })
        }
        else { None }
    }
}

impl<'a, 'b, T> Add<&'b Vector<T>> for &'a Vector<T>
    where T: Add<T, Output = T> + Copy
{
    type Output = Option<Vector<T>>;

    fn add(self, o: &'b Vector<T>) -> Self::Output {
        if self.len() == o.len() {
            Some(Vector {
                v: self.v.iter().enumerate().map(|i|  *i.1 + o[i.0]).collect(),
            })
        }
        else { None }
    }
}

impl<T> Sub for Vector<T>
    where T: Sub<T, Output = T> + Copy
{
    type Output = Option<Self>;

    fn sub(self, o: Self) -> Self::Output {
        if self.len() == o.len() {
            Some(Self {
                v: self.v.iter().enumerate().map(|i|  *i.1 - o[i.0]).collect(),
            })
        }
        else { None }
    }
}

impl<'a, 'b, T> Sub<&'b Vector<T>> for &'a Vector<T>
    where T: Sub<T, Output = T> + Copy
{
    type Output = Option<Vector<T>>;

    fn sub(self, o: &'b Vector<T>) -> Self::Output {
        if self.len() == o.len() {
            Some(Vector {
                v: self.v.iter().enumerate().map(|i|  *i.1 - o[i.0]).collect(),
            })
        }
        else { None }
    }
}

impl<T> Neg for Vector<T>
    where T: Neg<Output = T> + Copy
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            v: self.v.iter().map(|i|  -*i).collect(),
        }
    }
}

impl<'a, T> Neg for &'a Vector<T>
    where T: Neg<Output = T> + Copy
{
    type Output = Vector<T>;

    fn neg(self) -> Self::Output {
        Vector {
            v: self.v.iter().map(|i|  -*i).collect(),
        }
    }
}

impl<T, U> Mul<U> for Vector<T>
    where T: Mul<U, Output = T> + Copy,
          U: Copy
{
    type Output = Self;

    fn mul(self, o: U) -> Self::Output {
        Self {
            v: self.v.iter().map(|e| *e * o).collect(),
        }
    }
}

impl<'a, T, U> Mul<U> for &'a Vector<T>
    where T: Mul<U, Output = T> + Copy,
          U: Copy
{
    type Output = Vector<T>;

    fn mul(self, o: U) -> Self::Output {
        Vector {
            v: self.v.iter().map(|e| *e * o).collect(),
        }
    }
}

impl<T> Display for Vector<T>
    where T: Display 
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for i in self.v.iter().enumerate() {
            if i.0 != 0 { s.push_str(", "); }
            s.push_str(format!("{}", i.1).as_str());
        }
        write!(f, "<{}>", s)
    }
}

impl<T> Vector<T> {
    pub fn new(v: Vec<T>) -> Self {
        Self { v }
    }
    pub fn len(&self) -> usize {
        self.v.len()
    }
}