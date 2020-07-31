use std::{
    ops::{Add, Sub, Mul, Neg, Index, IndexMut},
    fmt::{self, Display, Formatter},
    cmp::PartialEq,
    iter::IntoIterator,
    vec::IntoIter,
    slice::{Iter, IterMut},
    convert::{From, TryFrom},
};
use crate::{Matrix, INDEX_START/*, diff_len::**/};

#[derive(PartialEq, Debug, Clone)]
pub struct Vector<T> {
    v: Vec<T>
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.v[i - INDEX_START]
    }
}

impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.v[i - INDEX_START]
    }
}

impl<T> Add for Vector<T>
    where T: Add<T, Output = T> + From<i32> + Copy
{
    type Output = Self;

    fn add(self, o: Self) -> Self::Output {
        if self.len() == o.len() {
            Self {
                v: self.v.iter().enumerate().map(|i|  *i.1 + o[i.0 + INDEX_START]).collect(),
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
                v: self.v.iter().enumerate().map(|i|  *i.1 + o[i.0 + INDEX_START]).collect(),
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
                v: self.v.iter().enumerate().map(|i|  *i.1 - o[i.0 + INDEX_START]).collect(),
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
                v: self.v.iter().enumerate().map(|i|  *i.1 - o[i.0 + INDEX_START]).collect(),
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
        write!(f, "<")?;
        for i in self.v.iter().enumerate() {
            if i.0 != 0 { write!(f, ", ")? }
            write!(f, "{}", i.1)?;
        }
        write!(f, ">")
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
            Ok(Self::from(value.column_clone(0)))
        }
        else if value.get_rows() == 1 {
            Ok(Self::from(value.row_clone(0)))
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
    use super::{Vector, Matrix, TryFrom, IntoIter};
    const A: [i32; 2] = [1, 2];
    const B: [i32; 2] = [3, 5];

    fn v(arr: [i32; 2]) -> Vector<i32> {
        Vector::from(Vec::from(arr))
    }

    fn assert_eq_iter<T: PartialEq>(mut a: IntoIter<T>, mut b: IntoIter<T>) {
        assert!(a.all(|e| e == b.next().unwrap()));
    }

    #[test]
    fn assert_eq_iter_test() {
        let v = vec![1, 2, 3];
        assert_eq_iter(v.clone().into_iter(), v.into_iter());
    }

    #[test]
    fn index() {
        let a = v(A);
        let b = v(B);
        assert_eq!(a[1], 1);
        assert_eq!(a[2], 2);
        assert_eq!(b[1], 3);
        assert_eq!(b[2], 5);
    }

    #[test]
    fn index_mut() {
        let mut c = v(A);
        c[1] = 3;
        assert_eq!(c, v([3, 2]));
    }

    #[test]
    fn add() {
        assert_eq!(v(A) + v(B), v([4, 7]));
    }

    #[test]
    fn add_ref() {
        assert_eq!(&v(A) + &v(B), v([4, 7]))
    }

    #[test]
    fn sub() {
        assert_eq!(v(A) - v(B), v([-2, -3]));
    }

    #[test]
    fn sub_ref() {
        assert_eq!(&v(A) - &v(B), v([-2, -3]));
    }

    #[test]
    fn neg() {
        assert_eq!(-v(A), v([-1, -2]));
        assert_eq!(-v(B), v([-3, -5]));
    }

    #[test]
    fn neg_ref() {
        assert_eq!(-&v(A), v([-1, -2]));
        assert_eq!(-&v(B), v([-3, -5]));
    }

    #[test]
    fn mul() {
        assert_eq!(v(A) * 2, v([2, 4]));
        assert_eq!(v(B) * -3, v([-9, -15]));
    }

    #[test]
    fn mul_ref() {
        assert_eq!(&v(A) * 2, v([2, 4]));
        assert_eq!(&v(B) * -3, v([-9, -15]));
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", v(A)), "<1, 2>");
        assert_eq!(format!("{}", v(B)), "<3, 5>");
    }

    #[test]
    fn into_iter() {
        let vec_a = v(A).into_iter();
        let arr_a = Vec::from(A).into_iter();
        assert_eq_iter(vec_a, arr_a);

        let vec_b = v(B).into_iter();
        let arr_b = Vec::from(B).into_iter();
        assert_eq_iter(vec_b, arr_b);
    }

    /*#[test]
    fn iter() {
        let vec_a = v(A).iter();
        let arr_a = Vec::from(A).iter();
        assert!(vec_a.all(|e| e == arr_a.next().unwrap()));

        let vec_b = v(B).iter();
        let arr_b = Vec::from(B).iter();
        assert!(vec_b.all(|e| e == arr_b.next().unwrap()));
    }*/

    /*#[test]
    fn iter_mut() {
        let a = v(A).iter_mut();
        for e in a {
            *e += 1;
        }
        assert!(a.all(|e))
    }*/

    #[test]
    fn from_vec() {
        assert_eq!(Vector::from(vec![1, 2]), Vector{v: Vec::from(A)});
        assert_eq!(Vector::from(vec![3, 5]), Vector{v: Vec::from(B)});
    }

    #[test]
    fn try_from_matrix() {
        assert_eq!(Vector::try_from(Matrix::from(v(A))), Ok(v(A)));
        assert_eq!(Vector::try_from(Matrix::square(vec![1, 2, 3, 4], 2)), Err("Cannot convert 2D matrix to vector"));
    }
}