use std::{
    ops::{Add, Sub, Mul, Neg, Index, IndexMut},
    cmp::PartialEq,
    convert::{From, TryFrom},
    borrow::ToOwned,
    fmt::{self, Display, Formatter},
    string::ToString,
};
use crate::{Vector, INDEX_START};

#[derive(PartialEq, Debug, Clone)]
pub struct Matrix<T> {
    m: Vec<Vector<T>>,
    rows: usize,
    columns: usize,
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, i: (usize, usize)) -> &Self::Output {
        &self.m[i.1 - INDEX_START][i.0]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, i: (usize, usize)) -> &mut Self::Output {
        &mut self.m[i.1][i.0]
    }
}

impl<T> Add for Matrix<T>
    where T: Add<T, Output = T> + From<i32> + Copy
{
    type Output = Option<Self>;

    fn add(self, o: Self) -> Self::Output {
        if self.rows == o.rows && self.columns == o.columns {
            Some(Self {
                m: self.m.iter().enumerate().map(|i|  (i.1 + &o.m[i.0])).collect(),
                ..self
            })
        }
        else {
            None 
        }
    }
}

impl<T> Sub for Matrix<T>
    where T: Sub<T, Output = T> + From<i32> + Copy
{
    type Output = Option<Self>;

    fn sub(self, o: Self) -> Self::Output {
        if self.rows == o.rows && self.columns == o.columns {
            Some(Self {
                m: self.m.iter().enumerate().map(|i|  (i.1 - &o.m[i.0])).collect(),
                ..self
            })
        }
        else { None }
    }
}

impl<T> Neg for Matrix<T>
    where T: Neg<Output = T> + Copy
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            m: self.m.iter().map(|i|  -i).collect(),
            ..self
        }
    }
}

impl<T, U> Mul<U> for Matrix<T>
    where T: Mul<U, Output = T> + Copy,
          U: Copy
{
    type Output = Self;

    fn mul(self, o: U) -> Self::Output {
        Self {
            m: self.m.iter().map(|e| e * o).collect(),
            ..self
        }
    }
}

impl<T> Mul<Vector<T>> for Matrix<T>
    where T: Add<T, Output = T> + Mul<T, Output = T> + From<i32> + Copy
{
    type Output = Option<Vector<T>>;

    fn mul(self, o: Vector<T>) -> Self::Output {
        if self.rows == o.len() {
            if self.rows == 0 {
                Some(Vector::from(Vec::with_capacity(0)))
            }
            else {
                let mut vecs: Vec<Vector<T>> = self.m.iter().enumerate().map(|i| i.1 * o[i.0]).collect();
                let mut builder = vecs.remove(0);
                for i in vecs.iter() {
                    builder = &builder + i;
                }
                Some(builder)
            }
        }
        else { None }
    }
}

impl<T> From<Vector<T>> for Matrix<T> {
    fn from(v: Vector<T>) -> Self {
        Self {
            rows: v.len(), // This need to be before m because m moves v into the Vec
            m: vec![v],
            columns: 1,
        }
    }
}

impl<T> TryFrom<Vec<Vector<T>>> for Matrix<T>
    where T: Copy {
    type Error = &'static str;

    fn try_from(v: Vec<Vector<T>>) -> Result<Self, Self::Error> {
        if v.len() == 0 || v.len() == 1 {
            Ok(Self {
                rows: v.len(),
                columns: v.len(),
                m: v,
            })
        }
        else {
            let len = v[0].len();
            if v.iter().all(|vec| vec.len() == len) {
                let mut r = Self {
                    columns: v.len(),
                    m: v,
                    rows: len,
                };
                r.transpose();
                Ok(r)
            }
            else {
                Err("Cannot make Matrix from a jagged Vec")
            }
        }
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Matrix<T>
    where T: Copy
{
    type Error = &'static str;

    fn try_from(v: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        Self::try_from(v.iter().map(|vec| Vector::from(vec.to_owned())).collect::<Vec<Vector<T>>>())
    }
}

impl<T> Display for Matrix<T>
    where T: ToString
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.rows == 0 {
            write!(f, "[]")
        }
        else {
            let mut strs: Vec<Vec<String>> = Vec::with_capacity(self.rows);
            for i in 0..self.rows {

                strs.push(self.row(i + INDEX_START).iter().map(|e| e.to_string()).collect());
            }
            if self.rows == 1 {
                write!(f, "[")?;
                for i in strs[0].iter() {
                    write!(f, " {} ", i)?;
                }
                write!(f, "]")
            }
            else {
                let mut sizes = vec![0; self.columns];
                for row in strs.iter() {
                    for e in row.iter().enumerate() {
                        sizes[e.0] = sizes[e.0].max(e.1.len());
                    }
                }
                let mut spaces = 0;
                for size in sizes.iter() {
                    spaces += size + 2;
                }
                write!(f, " _{}_ \n", " ".repeat(spaces))?;
                for row in strs.iter().enumerate() {
                    let padding = if row.0 == strs.len() - 1 { "_" } else {" "};
                    write!(f, "|{}", padding)?;
                    for s in row.1.iter().enumerate() {
                        write!(f, " {:^width$} ", s.1, width = sizes[s.0])?;
                    }
                    write!(f, "{}|\n", padding)?;
                }
                Ok(())
            }
        }
    }
}

impl<T> Matrix<T> {
    pub fn get_rows(&self) -> usize {
        self.rows
    }
    pub fn get_columns(&self) -> usize {
        self.columns
    }
    pub fn row(&self, i: usize) -> Vec<&T> {
        self.m.iter().map(|e| &e[i]).collect()
    }
    pub fn column(&self, i: usize) -> Vec<&T> {
        self.m[i - INDEX_START].vec().to_owned().iter().collect()
    }
    pub fn row_mut(&mut self, i: usize) -> Vec<&mut T> {
        self.m.iter_mut().map(|e| &mut e[i]).collect()
    }
    pub fn column_mut(&mut self, i: usize) -> Vec<&mut T> {
        self.m[i - INDEX_START].iter_mut().collect()
    }
    pub fn is_square(&self) -> bool {
        self.rows == self.columns
    }
}
impl<T> Matrix<T>
    where T: Copy
{
    pub fn row_clone(&self, i: usize) -> Vec<T> {
        self.m.iter().map(|e| e[i]).collect()
    }
    pub fn column_clone(&self, i: usize) -> Vec<T> {
        self.m[i - INDEX_START].vec().to_owned()
    }
}

impl<T> Matrix<T>
    where T: Copy
{
    pub fn transpose(&mut self) {
        let mut columns: Vec<Vector<T>> = Vec::with_capacity(self.columns);
        for i in 0..self.rows {
            columns.push(self.row_clone(i + INDEX_START).into());
        }
        *self = Self {
            m: columns,
            columns: self.rows,
            rows: self.columns,
        };
    }
}

impl<T> Matrix<T>
    where T: From<i32> + Clone,
{
    pub fn identity(size: usize) -> Self {
        let mut m: Vec<Vector<T>> = vec![vec![0.into(); size].into(); size];
        for i in m.iter_mut().enumerate() {
            i.1[i.0] = 1.into()
        }
        Self {
            m,
            columns: size,
            rows: size,
        }
    }
}

impl<T> Matrix<T>
    where T: Copy + Clone
{
    pub fn from_vec(v: Vec<T>, rows: usize, columns: usize) -> Self {
        if columns * rows != v.len() {
            panic!("A {}x{} vector must have {} elements, received Vec with {} elements", rows, columns, rows * columns, v.len());
        }
        else {
            let mut vecs = Vec::with_capacity(rows);
            for i in 0..rows {
                vecs.push(Vec::from(&v[columns*i..columns*(i + 1)]));
            }
            Self::try_from(vecs).unwrap()
        }
    }
    pub fn square(v: Vec<T>) -> Self {
        let s = (v.len() as f32).sqrt();
        if s % 1.0 != 0.0 {
            panic!("A square matrix can only be made from a Vec whose length is a perfect square, received length {}", v.len());
        }
        Self::from_vec(v, s as usize, s as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::{Vector, Matrix};
    type TestMatrix = [i32; 4];
    const A: TestMatrix = [1, 2, 3, 4];
    const B: TestMatrix = [5, 6, 7, 8];

    fn m(a2d: TestMatrix) -> Matrix<i32> {
        Matrix::square(Vec::from(&a2d[0..4]))
    }

    mod constructors {
        use super::*;

        #[test]
        fn test_m() {
            assert_eq!(Matrix{ m: vec![Vector::from(vec![1, 3]), Vector::from(vec![2, 4])], rows: 2, columns: 2}, m(A));
            assert_eq!(Matrix{ m: vec![Vector::from(vec![5, 7]), Vector::from(vec![6, 8])], rows: 2, columns: 2}, m(B));
        }

        #[test]
        fn from_vector() {
            assert_eq!(Matrix::from_vec(A.into(), 2, 2), m(A));
            assert_eq!(
                Matrix::from_vec(vec![1, 2, 3, 4, 5, 6], 3, 2), 
                Matrix{ m: vec![Vector::from(vec![1, 3, 5]), Vector::from(vec![2, 4, 6])], rows: 3, columns: 2}
            );
        }

        #[test]
        fn try_from_vec_vector() {
            
        }

        #[test]
        fn try_from_vec_vec() {

        }
    }
}