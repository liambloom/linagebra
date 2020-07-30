use std::{
    ops::{Add, Sub, Mul, Neg, Index, IndexMut},
    fmt::{self, Display, Formatter},
    cmp::PartialEq,
    iter::IntoIterator,
    vec::IntoIter,
    slice::{Iter, IterMut},
    convert::{From, TryFrom},
};
use crate::{Matrix/*, diff_len::**/};

#[derive(PartialEq, Debug, Clone)]
pub struct Vector<T> {
    v: Vec<T>
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.v[i - 1]
    }
}

impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.v[i]
    }
}

impl<T> Add for Vector<T>
    where T: Add<T, Output = T> + From<i32> + Copy
{
    type Output = Self;

    fn add(self, o: Self) -> Self::Output {
        if self.len() == o.len() {
            Self {
                v: self.v.iter().enumerate().map(|i|  *i.1 + o[i.0]).collect(),
            }
        }
        else {
            //Self::diff_len().run("vectors", "add");
            let (a, b) = Self::same_len_copies(&self, &o);
            a + b
        }
    }
}

impl<'a, T> Add<&'a Vector<T>> for &'a Vector<T>
    where T: Add<T, Output = T> + From<i32> + Copy
{
    type Output = Vector<T>;

    fn add(self, o: &'a Vector<T>) -> Self::Output {
        if self.len() == o.len() {
            Vector {
                v: self.v.iter().enumerate().map(|i|  *i.1 + o[i.0]).collect(),
            }
        }
        else { 
            //Vector::<T>::diff_len().run("vectors", "add");
            let (a, b) = Vector::same_len_copies(self, o);
            a + b
         }
    }
}

impl<T> Sub for Vector<T>
    where T: Sub<T, Output = T> + From<i32> + Copy
{
    type Output = Self;

    fn sub(self, o: Self) -> Self::Output {
        if self.len() == o.len() {
            Self {
                v: self.v.iter().enumerate().map(|i|  *i.1 - o[i.0]).collect(),
            }
        }
        else { 
            //Self::diff_len().run("vectors", "add");
            let (a, b) = Self::same_len_copies(&self, &o);
            a - b
         }
    }
}

impl<'a, T> Sub<&'a Vector<T>> for &'a Vector<T>
    where T: Sub<T, Output = T> + From<i32> + Copy
{
    type Output = Vector<T>;

    fn sub(self, o: &'a Vector<T>) -> Self::Output {
        if self.len() == o.len() {
            Vector {
                v: self.v.iter().enumerate().map(|i|  *i.1 - o[i.0]).collect(),
            }
        }
        else { 
            //Vector::<T>::diff_len().run("vectors", "add");
            let (a, b) = Vector::same_len_copies(self, o);
            a - b
        }
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

impl<T> IntoIterator for Vector<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.v.into_iter()
    }
}

impl<T> TryFrom<Matrix<T>> for Vector<T>
    where T: Copy {
    type Error = &'static str;

    fn try_from(value: Matrix<T>) -> Result<Self, Self::Error> {
        if value.get_columns() == 1 {
            Ok(Self::from(value.column(0)))
        }
        else if value.get_rows() == 1 {
            Ok(Self::from(value.row(0)))
        }
        else {
            Err("Cannot convert 2D matrix to vector")
        }
    }
}

impl<T> From<Vec<T>> for Vector<T> {
    fn from(v: Vec<T>) -> Self {
        Self { v }
    }
}

impl<T> Vector<T> {
    pub fn len(&self) -> usize {
        self.v.len()
    }
    pub fn vec(&self) -> &Vec<T> {
        &self.v
    }
    pub fn iter(&self) -> Iter<T> {
        self.v.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.v.iter_mut()
    }
}

impl<T> Vector<T>
    where T: From<i32>
{
    fn make_len(&mut self, len: usize) {
        while len > self.len() {
            self.v.push(0.into());
        }
    }
}

impl<T> Vector<T>
    where T: From<i32> + Clone
{
    fn same_len_copies<'a>(a: &'a Self, b: &'a Self) -> (Self, Self) {
        let mut a = a.to_owned();
        let mut b = b.to_owned();
        if a.len() > b.len() {
            a.make_len(b.len());
        }
        else {
            b.make_len(a.len());
        }
        (a, b)
    }
    
}

#[cfg(test)]
mod tests {
    use super::Vector;
    const a: Vector<i32> = Vector{ v: vec![1, 2] };
    const b: Vector<i32> = Vector{ v: vec![3, 5] };

    #[test]
    fn constructor() {
        assert_eq!(Vector::from(vec![1, 2]), a);
        assert_eq!(Vector::from(vec![3, 5]), b);
    }

    #[test]
    fn add() {
        assert_eq!(a + b, Vector::from(vec![4, 7]));
    }
}